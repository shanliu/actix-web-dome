pub(crate) mod inoutput;
pub(crate) mod client;
pub(crate) mod log;
pub(crate) mod mysql;
pub(crate) mod redis;
pub(crate) mod upload;
pub(crate) mod ws;
pub(crate) mod tpl;

use actix_web::{Result, web,  HttpResponse, error::ResponseError, HttpRequest, Responder, http::StatusCode};
use sqlx::{
    MySql,
    Pool
};
use actix::{Addr, MailboxError};
use actix_redis::RedisActor;
use actix_files::NamedFile;
use serde::Serialize;
use serde_json::{json, to_string_pretty};
use futures::future::{ready,Ready};
use std::fmt::{Display, Formatter, Result as FmtResult};
use actix_web::error::PayloadError;

use std::sync::{Arc};
use crate::daos::Database;

// 全局数据

pub struct AppState<'c> {
    pub context: Arc<Database<'c>>,
    pub app_name: String,
    pub db_pool:Pool<MySql>,
    pub redis:Addr<RedisActor>
}

//统一错误
#[derive(Debug,Serialize)]
pub struct WebHandError{
    status:u16,
    message: String
}
impl WebHandError{
    fn new(msg:String)->WebHandError{
        return WebHandError{
            status:500,
            message:msg.to_string()
        };
    }
}
impl Display for WebHandError {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        write!(f, "{}", to_string_pretty(self).unwrap())
    }
}
impl ResponseError for WebHandError {
    // builds the actual response to send back when an error occurs
    fn error_response(&self) -> web::HttpResponse {
        web::HttpResponse::Ok().json(self)
    }
}
impl From<PayloadError> for WebHandError{
    fn from(err:PayloadError) -> Self {
        return WebHandError::new(format!("{:?}",err))
    }
}
impl From<reqwest::Error> for WebHandError{
    fn from(err:reqwest::Error) -> Self {
        return WebHandError::new(format!("{:?}",err))
    }
}
impl From<actix_web::Error> for WebHandError{
    fn from(err:actix_web::Error) -> Self {
        return WebHandError::new(format!("{:?}",err))
    }
}
impl From<tera::Error> for WebHandError{
    fn from(err:tera::Error) -> Self {
        return WebHandError::new(format!("{:?}",err))
    }
}
impl From<sqlx::Error> for WebHandError{
    fn from(err: sqlx::Error) -> Self {
        match err {
            sqlx::Error::RowNotFound=>{
                return WebHandError::new("结果不存在".to_string())
            },
            _err@_ => {
                return WebHandError::new(format!("{:?}",_err))
            },
        }
    }
}
impl From<actix_redis::Error> for WebHandError{
    fn from(err:actix_redis::Error) -> Self {
        match err {
            actix_redis::Error::NotConnected=>{
                return WebHandError::new("结果不存在".to_string())
            },
            _err@_ => {
                return WebHandError::new(format!("{:?}",_err))
            },
        }
    }
}
impl From<MailboxError> for WebHandError{
    fn from(err:MailboxError) -> Self {
        return WebHandError::new(format!("{:?}",err))
    }
}

pub(crate) trait HttpResponseOKJSON{
    fn json<T:Serialize>(value:T)->HttpResponse;
}
impl HttpResponseOKJSON for HttpResponse{
    fn json<T:Serialize>(value:T)->HttpResponse{
        HttpResponse::Ok().json(json!({
            "status":0,
            "data":value
        }))
    }
}

// 统一json输出对象
pub struct WebJSONResult{
    data:HttpResponse
}
impl WebJSONResult{
    pub fn new<T:Serialize>(value:T)->Self{
        return WebJSONResult{
            data:HttpResponse::Ok().json(json!({
                "status":0,
                "data":value
            }))
        };
    }
}
impl Responder for WebJSONResult
{
    type Error = WebHandError;
    type Future = Ready<Result<HttpResponse, WebHandError>>;
    fn respond_to(self, _: &HttpRequest) -> Self::Future {
        ready(Ok(self.data))
    }
}


//默认错误页面
pub(crate) async fn p404() -> Result<NamedFile> {
    Ok(NamedFile::open("static/404.html")?.set_status_code(StatusCode::NOT_FOUND))
}
