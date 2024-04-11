use actix_web::{get, web, HttpResponse, Scope};
use base64::encode;
use sea_orm::{IntoActiveModel};
use serde::{Deserialize, Serialize};
use crate::controller::{PublicErrorResponse, PublicErrorCode};
use serde_json::{Value, json};
use super::{AppState, PublicSuccessCode, PublicSuccessResponse};

#[derive(Debug)]
pub struct SoftwaresController;
impl SoftwaresController {
    pub fn new() -> Scope {
        let x: Scope = web::scope("/softwares").service(get_file)
        .service(get_information).service(get_version_list).service(add_software)
        .service(delete_software).service(update_software);
        x
    }
}

#[derive(Deserialize)]
struct TriggerQuery {
    archive: String,
    version: String,
}
#[get("/file")]
async fn get_file(
    app_data: web::Data<AppState>,
    query: web::Query<TriggerQuery>,
) -> HttpResponse {
    // 读取请求中的查询条件
    let archive  = &query.archive;
    let version = &query.version;
    let db = app_data.db_conn();
    // 获得文件URL
    let url_res = crate::service::softwares_service::SoftwaresService::get_file_url_by_archive_version(archive, version, db).await;
    match url_res {
        Ok(url) => {
            let file = crate::service::file_service::FileService::get_file_by_url(url);
            match file {
                // 发送文件
                // 将 Vec<u8> 编码为 Base64 字符串
                Ok(vec) => {
                    //return HttpResponse::Ok().json(encode(vec));
                    return PublicSuccessResponse::new(PublicSuccessCode::ValidResponse, encode(vec)).to_json_response()
                } ,
                Err(e) => return e.to_public_error().to_json_response(),
            }
            
        }
        Err(e) => return e.to_public_error().to_json_response(),
    }
    
}

#[get("/information")]
async fn get_information(
    app_data: web::Data<AppState>,
    query: web::Query<TriggerQuery>,
) -> HttpResponse {
    let archive  = &query.archive;
    let version = &query.version;
    let db = app_data.db_conn();
    let res = crate::service::softwares_service::SoftwaresService::get_information_by_archive_version(archive, version, db).await;
    match res {
        Ok(s) => return HttpResponse::Ok().json(s),
        Err(e) => return e.to_public_error().to_json_response(),
    }
}
#[get("/version_list")]
async fn get_version_list(
    app_data: web::Data<AppState>,
    query: web::Query<TriggerQuery>,
) -> HttpResponse {
    let archive  = &query.archive;
    let db = app_data.db_conn();
    let res = crate::service::softwares_service::SoftwaresService::get_version_list_by_archive(archive, db).await;
    match res {
        Ok(s) => return HttpResponse::Ok().json(s),
        Err(e) => return e.to_public_error().to_json_response(),
    }
}
// 管理员操作

#[derive(Clone, Debug, PartialEq, Eq,Deserialize, Serialize)]
pub struct SoftwareQuery {
    pub archive: String,
    pub version: String,
    pub version_major: i32,
    pub version_minor: i32,
    pub version_patch: i32,
    pub component: String,
    pub origin: String,
    pub label: String,
    pub architecture: String,
    pub download: String,
    pub others: String,
    pub flag: i32,
}

#[get("/add_software")]
async fn add_software(
    app_data: web::Data<AppState>,
    query: web::Query<crate::entities::softwares::Model>,
) -> HttpResponse {
    let db = app_data.db_conn();
    let active_software = <crate::entities::softwares::Model as Clone>::clone(&query).into_active_model();
    let res = crate::service::softwares_service::SoftwaresService::add_software(&active_software, db).await;
    match res {
        Ok(s) => return HttpResponse::Ok().json(s),
        Err(e) => return e.to_public_error().to_json_response(),
    }
}

#[get("/delete_software")]
async fn delete_software(
    app_data: web::Data<AppState>,
    query: web::Query<TriggerQuery>,
) -> HttpResponse {
    let db = app_data.db_conn();
    let res = crate::service::softwares_service::SoftwaresService::delete_software(&query.archive, &query.version, db).await;
    match res {
        Ok(s) => return HttpResponse::Ok().json(s.rows_affected),
        Err(e) => return e.to_public_error().to_json_response(),
    }
}

#[get("/update_software")]
async fn update_software(
    app_data: web::Data<AppState>,
    query: web::Query<crate::entities::softwares::Model>,
) -> HttpResponse {
    let db = app_data.db_conn();
    if query.archive.is_empty() || query.version.is_empty() {
        return PublicErrorResponse::new(PublicErrorCode::InvalidParameter, "".to_string()).to_json_response();
    }
    let res = crate::service::softwares_service::SoftwaresService::update_software(&query, db).await;
    match res {
        Ok(s) => return HttpResponse::Ok().json(s),
        Err(e) => return e.to_public_error().to_json_response(),
    }
}