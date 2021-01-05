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
use crate::handler::WebHandError;
use crate::service::account::AccountSerice;


#[derive(Serialize, Deserialize)]
struct MyObj {
    name: String,
}
#[derive(Deserialize)]
pub struct AuthRequest {
    id: u64
}




#[get("/redis/set/{key}/{val}")]
pub(crate) async fn index(web::Path((key,val)):web::Path<(String, String)>,data: web::Data<WebData>) -> Result<HttpResponse,WebHandError> {
    let one = data.redis.send(Command(resp_array!["SET",key.as_str(),val.as_str()]));

    let b=one.await?;

    Ok(HttpResponse::Ok().json(json!({
        "cat":key,
        "id":val
    })))
}
