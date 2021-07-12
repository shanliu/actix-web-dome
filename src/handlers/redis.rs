use actix_web::{get, web, HttpResponse};
use serde_json::json;
use actix_redis::Command;
use crate::handlers::WebHandError;
use crate::AppState;


#[get("/redis/set/{key}/{val}")]
pub(crate) async fn index<'a>(key:web::Path<String>,val:web::Path<String>,data: web::Data<AppState<'a>>) -> Result<HttpResponse,WebHandError> {
    // let comm:redis_async::resp::RespValue=resp_array!["SET",key.as_str(),val.as_str()];
    //
    let one = data.redis.send(Command("set aa bbb".into()));
    //
    let b=one.await?.is_ok();

    Ok(HttpResponse::Ok().json(json!({
        // "cat":key,
        // "id":val,
        // "ret":b,
    })))
}
