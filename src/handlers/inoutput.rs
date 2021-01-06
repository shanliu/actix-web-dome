use actix_web::{get, web, HttpRequest};
use serde::{Deserialize};
use actix_web::{ Result};
use serde_json::json;
use crate::handlers::{WebHandError, WebJSONResult};

#[derive(Deserialize)]
pub struct AuthRequest {
    id: u64
}

#[get("/")]
pub(crate) async fn index(web::Query(info):web::Query<AuthRequest>) ->Result<WebJSONResult,WebHandError> {
    Ok(WebJSONResult::new(json!({
        "ss":info.id
    })))
}

#[get("/http_req")]
pub(crate) async fn req(req: HttpRequest) ->Result<WebJSONResult,WebHandError> {
    Ok(WebJSONResult::new(json!({
        "ss":"sss"
    })))
}
