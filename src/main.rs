mod entities;
mod dao;
mod controller;
mod service;
mod test;
use actix_cors::Cors;
use serde_derive::Serialize;
use simple_logger::{SimpleLogger};
use actix_session::{
    config::{PersistentSession, SessionMiddlewareBuilder},
    storage::RedisActorSessionStore,
    SessionMiddleware,
};
use actix_web::{
    cookie::{time::Duration, Key},
    get,
    http::header,
    post, web, App, HttpResponse, HttpServer, Responder,
};
use dotenv::dotenv;
use std::env;
use crate::controller::{
    softwares_controller::SoftwaresController
};
// 返回的类要转换成HttpResponse，实现trait
#[derive(Serialize)]
struct MyData {
    message: String,
    num : i32,
}
impl MyData {
    pub fn new(message: &str, num: i32) -> Self {
        MyData { message: message.to_string(), num }
    }
    pub fn to_json_response(&self) -> HttpResponse {
        HttpResponse::Ok().json(self)
    }
}

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    print!("{:?}", req_body);
    MyData::new(&req_body, 999).to_json_response()
}

async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}

fn logger_init() {
    // 初始化日志系统，日志级别为Info
    // 如果需要调试，可以将日志级别设置为Debug
    let logger = SimpleLogger::new().with_level(log::LevelFilter::Info);

    logger.init().unwrap();
}
fn init_session_middleware() -> SessionMiddleware<RedisActorSessionStore> {
    const SECS_IN_WEEK: i64 = 7 * 24 * 60 * 60;
    let builder: SessionMiddlewareBuilder<RedisActorSessionStore> = SessionMiddleware::builder(
        RedisActorSessionStore::new(env::var("REDIS_URL").expect("REDIS_URL not set!").as_str()),
        Key::from(
            env::var("SESSION_KEY")
                .expect("SESSION_KEY not set!")
                .as_bytes(),
        ),
    )
    .cookie_secure(true)
    .cookie_same_site(actix_web::cookie::SameSite::None)
    // cookie的有效期为一周
    .session_lifecycle(PersistentSession::default().session_ttl(Duration::seconds(SECS_IN_WEEK)));

    return builder.build();
}
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // 调用 dotenv 函数来加载 .env 文件
    dotenv().ok();
    logger_init();
    log::info!("Starting server...");
    let app_state = controller::AppState::new()
        .await
        .expect("Failed to create app state");

    HttpServer::new(move || {
        let cors = Cors::permissive()
            .allow_any_origin() // 允许任何来源
            .allowed_methods(vec!["GET", "POST"]) // 允许的方法
            .allowed_headers(vec![header::AUTHORIZATION, header::ACCEPT]) // 允许的头部
            .allowed_header(header::CONTENT_TYPE) // 允许的头部
            .supports_credentials() // 允许携带凭证
            .max_age(3600); // 预检请求的最大期限

        let session_middleware = init_session_middleware();

        let mut app = App::new()
            .wrap(cors)
            .wrap(session_middleware)
            .app_data(app_state.clone())
            .service(hello)
            .service(echo)
            .route("/hey", web::get().to(manual_hello));
        app = app.service( 
            web::scope("/api/v1")
                .service(SoftwaresController::new()),
        );
        // println!("{:?}", app);
        app
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}


