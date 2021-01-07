use actix_web::{get, web};
use serde::{Deserialize};
use actix_web::{ Result};
use serde_json::json;
use crate::handlers::{WebHandError, WebJSONResult};
use crate::utils::web_query::{QueryGetTrait, QueryGet};

#[get("/")]
pub(crate) async fn index() ->Result<WebJSONResult,WebHandError> {
    Ok(WebJSONResult::new(json!({
        "say":"hi"
    })))
}



#[get("/query_get")]
pub(crate) async fn query_get(query: web::Query<QueryGet>) ->Result<WebJSONResult,WebHandError> {
    let val=query.get_parse::<i32>("id")?;
    Ok(WebJSONResult::new(json!({
        "ss":val
    })))
}

#[derive(Deserialize)]
pub struct GetParam {
    id: u64
}
#[get("/get")]
pub(crate) async fn get(query: web::Query<GetParam>) ->Result<WebJSONResult,WebHandError> {
    Ok(WebJSONResult::new(json!({
        "ss":query.id
    })))
}

#[derive(Deserialize)]
pub struct GetParam1 {
    id: u64
}
#[get("/form")]
pub(crate) async fn form(query: web::Form<GetParam1>) ->Result<WebJSONResult,WebHandError> {
    Ok(WebJSONResult::new(json!({
        "ss":query.id
    })))
}
