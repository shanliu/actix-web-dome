use actix_web::{get, web, HttpResponse};
use serde_json::json;
use actix_redis::Command;
use redis_async::{ resp_array};
use crate::handlers::WebHandError;
use crate::AppState;


#[get("/redis/set/{key}/{val}")]
pub(crate) async fn index<'a>(web::Path((key,val)):web::Path<(String, String)>,data: web::Data<AppState<'a>>) -> Result<HttpResponse,WebHandError> {
    let one = data.redis.send(Command(resp_array!["SET",key.as_str(),val.as_str()]));

    let b=one.await?.is_ok();

    Ok(HttpResponse::Ok().json(json!({
        "cat":key,
        "id":val,
        "ret":b,
    })))
}
