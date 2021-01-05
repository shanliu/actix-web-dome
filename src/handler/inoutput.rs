use actix_web::{get, web, HttpResponse};
use crate::WebData;
use serde::{Deserialize, Serialize};
use actix_web::{error, Result,error::ResponseError};
use serde_json::json;
use sqlx::mysql::MySqlRow;
use sqlx::{Row, Error};
use actix_redis::Command;
use redis_async::{resp::RespValue, resp_array};
use serde::export::Formatter;
use crate::handler::{WebHandError, WebJSONResult};
use crate::service::account::AccountSerice;


#[derive(Serialize, Deserialize)]
struct MyObj {
    name: String,
}
#[derive(Deserialize)]
pub struct AuthRequest {
    id: u64
}


#[get("/")]
pub(crate) async fn index(web::Query(info):web::Query<AuthRequest>) ->Result<WebJSONResult,WebHandError> {
    Ok(WebJSONResult::new(json!({
        "ss":11
    })))
}
