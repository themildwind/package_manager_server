pub mod softwares_controller;
use std::time::Duration;
use actix_web::{web};
use serde::Serialize;



pub struct AppState {
    /// Database connection pool
    db: sea_orm::DatabaseConnection,

}

impl AppState {
    pub async fn new() -> Result<web::Data<Self>, sea_orm::DbErr> {
        let dburl = std::env::var("DATABASE_URL").expect("DATABASE_URL not set");
        let mut dbopts = sea_orm::ConnectOptions::new(dburl);

        dbopts
            .max_connections(100)
            .min_connections(2)
            .connect_timeout(Duration::from_secs(8))
            .acquire_timeout(Duration::from_secs(8))
            .idle_timeout(Duration::from_secs(30))
            .max_lifetime(Duration::from_secs(60))
            .sqlx_logging(false)
            .sqlx_logging_level(log::LevelFilter::Info);

        let db = sea_orm::Database::connect(dbopts)
            .await
            .expect("Failed to connect to database");

        let result: web::Data<AppState> = web::Data::new(Self {
            db,
        });
        return Ok(result);
    }


    /// 获取数据库连接
    pub fn db_conn(&self) -> &sea_orm::DatabaseConnection {
        &self.db
    }


}

#[derive(Debug)]
pub enum SystemError {
    DbErr(sea_orm::DbErr),
    /// 请在系统内部重试
    InnerRetry,
    Busy(String),
    IoErr,
    
}

impl SystemError {
    pub fn to_public_error(&self) -> PublicErrorResponse {
        log::warn!("SystemError to_public_error: {:?}", self);
        match self {
            SystemError::DbErr(_e) => {
                PublicErrorResponse::new(PublicErrorCode::InvalidParameter, format!("{:?}", _e.to_string()))
            }
            SystemError::Busy(_s) => {
                PublicErrorResponse::new(PublicErrorCode::InternalServerError, format!("系统繁忙"))
            }
            SystemError::InnerRetry => PublicErrorResponse::new(
                PublicErrorCode::InternalServerError,
                format!("系统内部错误"),
            ),
            SystemError::IoErr => PublicErrorResponse::new(
                PublicErrorCode::InternalServerError,
                format!("IO错误"),
            ),
            
        }
    }
}

impl From<sea_orm::DbErr> for SystemError {
    fn from(e: sea_orm::DbErr) -> Self {
        SystemError::DbErr(e)
    }
}

#[derive(Debug, Clone, Copy)]
pub enum PublicErrorCode {
    /// 未找到
    NotFound = 40404,
    /// 未授权
    Unauthorized = 40401,
    /// 无权限
    Forbidden = 40403,
    /// 参数错误
    InvalidParameter = 40000,
    /// 服务器错误
    InternalServerError = 50000,
}

impl Serialize for PublicErrorCode {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_u16(*self as u16)
    }
}

/// 暴露給外部的公共错误
#[derive(Debug, Serialize)]
pub struct PublicErrorResponse {
    status_code: PublicErrorCode,
    message: String,
}

impl PublicErrorResponse {
    pub fn new(status_code: PublicErrorCode, message: String) -> Self {
        Self {
            status_code,
            message,
        }
    }
    pub fn to_json_response(&self) -> actix_web::HttpResponse {
        let code = self.status_code as u16;
        let st = format!("{code}");
        let s = st.as_str();
        if s.bytes().nth(0).unwrap() == b'4' {
            return actix_web::HttpResponse::BadRequest().json(self);
        } else if s.bytes().nth(0).unwrap() == b'5' {
            return actix_web::HttpResponse::InternalServerError().json(self);
        } else {
            return actix_web::HttpResponse::Ok().json(self);
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum PublicSuccessCode {
    /// 有效返回
    ValidResponse = 30101,
    
}

impl Serialize for PublicSuccessCode {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_u16(*self as u16)
    }
}
/// 暴露給外部的公共成功返回
#[derive(Debug, Serialize)]
pub struct PublicSuccessResponse {
    status_code: PublicSuccessCode,
    data: String,
}

impl PublicSuccessResponse {
    pub fn new(status_code: PublicSuccessCode, data: String) -> Self {
        Self {
            status_code,
            data,
        }
    }
    pub fn to_json_response(&self) -> actix_web::HttpResponse {
        return actix_web::HttpResponse::Ok().json(self);
        
    }
}