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
use crate::handler::{WebHandError};
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
pub(crate) async fn index(web::Query(info):web::Query<AuthRequest>) ->Result<HttpResponse,WebHandError> {
    // trait Atest{}
    // struct A1test{}
    // impl Atest for A1test{}
    // let a:A1test=A1test{};
    // let c=&a as &Atest;
    // let b=1;
    // let t=&b as &serde::Serialize;
    // let c=Box::new(t);

    Ok(HttpResponse::Ok().body("ddd"))
}
