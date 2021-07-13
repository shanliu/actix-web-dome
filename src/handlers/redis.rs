use actix_web::{get, web, HttpResponse};
use serde_json::json;
use actix_redis::Command;
use crate::handlers::WebHandError;
use crate::AppState;
use redis_async::{resp_array};
#[get("/redis/set/{key}")]
pub(crate) async fn index<'a>(key:web::Path<String>,data: web::Data<AppState<'a>>) -> Result<HttpResponse,WebHandError> {
    let comm:redis_async::resp::RespValue=resp_array!["SET",key.as_str(),"ttt"];
    let one = data.redis.send(Command(comm));
    let b=one.await?.is_ok();
    Ok(HttpResponse::Ok().json(json!({
         "cat":key.to_string(),
         "ret":b,
    })))
}
