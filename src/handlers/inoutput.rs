use actix_web::{get,post, web};
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


//  curl  http://127.0.0.1:8080/query_get?id=1
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
//  curl  http://127.0.0.1:8080/get?id=1
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
// curl -d 'id=1'  http://127.0.0.1:8080/from
#[post("/from")]
pub(crate) async fn from(query: web::Form<GetParam1>) ->Result<WebJSONResult,WebHandError> {
    Ok(WebJSONResult::new(json!({
        "ss":query.id
    })))
}

use futures::StreamExt;
use actix_web::web::Buf;
#[get("/payload")]
pub(crate) async fn payload(mut body: web::Payload) ->Result<WebJSONResult,WebHandError> {
    // let resp = reqwest::get("https://httpbin.org/ip")
    //     .await?;


    let mut bytes = web::BytesMut::new();
    while let Some(item) = body.next().await {
        let item = item?;
        println!("Chunk: {:?}", &item);
        bytes.extend_from_slice(&item);
    }
    let tmp=bytes.bytes();
    Ok(WebJSONResult::new(json!({
        "ddd":String::from_utf8_lossy(tmp)
    })))
}
