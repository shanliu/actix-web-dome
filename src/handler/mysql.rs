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


#[derive(Deserialize)]
pub struct AuthRequest {
    id: u64
}

#[get("/db")]
pub(crate) async fn index(web::Query(info):web::Query<AuthRequest>,data: web::Data<WebData>) -> Result<HttpResponse,WebHandError> {
    let user=AccountSerice::new(&data.db).find_by_id(info.id as u32).await?;
    Ok(HttpResponse::Ok().json(json!({
        "cat":format!("Hello {}!",user.show_name()),
        "id":info.id
    })))
}