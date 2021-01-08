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
    id: u32
}
//  curl  http://127.0.0.1:8080/get?val=aaaaa
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

#[derive(Deserialize)]
pub struct JSONParam {
    id: u64
}
// curl -H "Content-Type: application/json" -X POST --data '{"id":10001}' http://localhost:8080/json
#[post("/json")]
pub(crate) async fn json(info: web::Json<JSONParam>) ->Result<WebJSONResult,WebHandError>{
    Ok(WebJSONResult::new(json!({
        "ss":info.id
    })))
}

use actix_web::web::Buf;
use futures::{StreamExt};
#[post("/payload")]
pub(crate) async fn payload(mut body: web::Payload) ->Result<WebJSONResult,WebHandError> {
    let mut bytes = web::BytesMut::new();
    while let Some(item) = body.next().await {
        let item = item?;
        println!("Chunk: {:?}", &item);
        bytes.extend_from_slice(&item);
    }
    let str=String::from_utf8(Vec::from(bytes.bytes())).unwrap();
    // let str:std::borrow::Cow<str>=String::from_utf8_lossy(bytes.bytes());
    Ok(WebJSONResult::new(json!({
        "ddd":str
    })))
}


#[get("/path/{id}")]
pub(crate) async fn path(web::Path(id):web::Path<u32,>,query: web::Query<QueryGet>) ->Result<WebJSONResult,WebHandError> {
    let val=query.get_string("id")?;
    Ok(WebJSONResult::new(json!({
        "path":id,
        "id":val
    })))
}

//
//
// use reqwest::Client;
// use actix_multipart::Multipart;
// use futures::{StreamExt, TryStreamExt};
//
// #[post("/multipart")]
// pub(crate) async fn multipart(mut body: Multipart) ->Result<WebJSONResult,WebHandError> {
//
//     let client = Client::new();
//     let builder = client.get("http://httpbin.org/get")
//         .body(reqwest::Body::wrap_stream(body));
//     let res=builder.send().await?.text().await?;
//
//     Ok(WebJSONResult::new(json!({
//         "ddd":res
//     })))
// }
