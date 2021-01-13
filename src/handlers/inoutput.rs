use actix_web::{get, post,dev, web, Error,HttpRequest, HttpResponse, HttpMessage, FromRequest};
use serde::{Deserialize};
use actix_web::{ Result};
use serde_json::json;
use crate::handlers::{WebHandError, WebJSONResult};
use crate::utils::web_query::{QueryGetTrait, QueryGet};
use actix_web::web::Buf;
use futures::{StreamExt};

use futures_util::future::{ok, err, Ready};
use reqwest::Client;
use actix_multipart::Multipart;
use futures::{TryStreamExt};
use actix_session::Session;
use actix_web::cookie::Cookie;
use actix_web::error::ErrorBadRequest;


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


#[derive(Debug, Deserialize)]
pub(crate) struct Thing {
    name: String
}
impl FromRequest for Thing {
    type Error = Error;
    type Future = Ready<Result<Self, Self::Error>>;
    type Config = ();

    fn from_request(req: &HttpRequest, _payload: &mut dev::Payload) -> Self::Future {
        ok(Thing { name: req.method().as_str().to_string() })
    }
}
#[get("/usertype")]
pub(crate) async fn usertype(supplied_thing: Option<Thing>) ->Result<WebJSONResult,WebHandError> {
    let out;
    match supplied_thing {
        // Puns not intended
        Some(thing) =>out= format!("Got something: {:?}", thing),
        None =>out= format!("No thing!")
    }
    Ok(WebJSONResult::new(json!({
        "ddd":out
    })))
}

//curl  -X POST --data 'xxxxxxxxxxxxx' http://localhost:8080/payload
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

// curl http://127.0.0.1:8080/path/111?id=11
#[get("/path/{id}")]
pub(crate) async fn path(web::Path(id):web::Path<u32,>,req: HttpRequest) ->Result<WebJSONResult,WebHandError> {
    let url = req.url_for("baidu", &["fack"]).unwrap();
    Ok(WebJSONResult::new(json!({
        "path":id,
        "url":url.into_string()
    })))
}

// curl http://127.0.0.1:8080/ruler/111?id=11
pub(crate) async fn ruler(web::Path(path_url):web::Path<String,>,req: HttpRequest) ->Result<WebJSONResult,WebHandError> {
    let url = req.url_for("path_name", &["myurl"]).unwrap();
    Ok(WebJSONResult::new(json!({
        "path":path_url,
        "url":url.into_string()
    })))
}


// curl http://127.0.0.1:8080/session
#[get("/session")]
pub(crate) async fn session(session: Session,req: HttpRequest) ->Result<WebJSONResult,WebHandError> {
    let mut counter = 1;
    if let Some(count) = session.get::<i32>("counter")? {
        println!("SESSION value: {}", count);
        counter = count + 1;
        session.set("counter", counter)?;
    } else {
        session.set("counter", counter)?;
    }
    Ok(WebJSONResult::new(json!({
        "counter":counter
    })))
}

// curl http://127.0.0.1:8080/cookie
#[get("/cookie")]
pub(crate) async fn cookie(req: HttpRequest) ->HttpResponse {
    let cookie:Cookie=req.cookie("nameq").unwrap_or(Cookie::new("nameq","hi"));
    let cookie1 = Cookie::build("nameb", "value")
       // .domain("www.rust-lang.org")
       // .path("/")
        .secure(true)
        .http_only(true)
        .finish();
    let cookie2=Cookie::new("bb","ccc");
    let mut res =HttpResponse::Ok().json(json!({
        "counter":cookie.value()
    }));
    res.add_cookie(&cookie);
    res.add_cookie(&cookie1);
    res.add_cookie(&cookie2);
    res
}


#[post("/multipart1")]
pub(crate) async fn multipart1(mut body: Multipart) ->Result<WebJSONResult,WebHandError> {

    // let (tx, mut rx) = tokio::sync::broadcast::channel::<actix_web::web::Bytes>(100);
    // tokio::task::spawn( async move {
    //     let stream = async_stream::stream! {
    //         while let Ok(value) = rx.recv().await {
    //             yield std::io::Result::Ok(value);
    //         }
    //     };
    //
    //     let res=actix_web::client::Client::new()
    //         .post("http://baidu.com")
    //         .send_stream(stream)
    //         ;
    //     println!("{}",res.await.unwrap().body().await.unwrap().to_vec().into());
    // });
    // let str="aaa".to_string();
    // tx.send(str.into()).unwrap();
    // let str1="aaa1".to_string();
    // tx.send(str1.into()).unwrap();
    Ok(WebJSONResult::new(json!({

    })))
}

#[post("/multipart2")]
pub(crate) async fn multipart2(mut body: Multipart) ->Result<WebJSONResult,WebHandError> {

    let (tx, mut rx) = tokio::sync::broadcast::channel::<String>(100);
    tokio::task::spawn( async move {
        let stream = async_stream::stream! {
            while let Ok(value) = rx.recv().await {
                yield std::io::Result::Ok(value);
            }
        };

        let client = Client::new();
        let builder = client.get("http://httpbin.org/get")
            .body(reqwest::Body::wrap_stream(stream));

        let res = builder.send().await.unwrap().text().await.unwrap();

        println!("{}",res);
    });
    let str="aaa".to_string();
    tx.send(str).unwrap();
    let str1="aaa1".to_string();
    tx.send(str1).unwrap();
    Ok(WebJSONResult::new(json!({

    })))
}
