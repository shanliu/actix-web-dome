use actix_web::{get, post, web};

use actix_web::{ Result};
use serde_json::json;
use crate::handlers::{WebHandError, WebJSONResult};

use actix_web::web::{Bytes};
use futures::{StreamExt};

use reqwest::Client;
use actix_multipart::Multipart;
use futures::{TryStreamExt};
use actix_web::error::{PayloadError};

//curl  -X POST --data 'xxxxxxxxxxxxx' http://localhost:8080/multipart1
// #[get("/multipart1")]
// pub(crate) async fn multipart1( body: web::Payload) ->Result<WebJSONResult,WebHandError> {
//     let res=actix_web::client::Client::new()
//         .post("http://127.0.0.1")
//         .send_stream(body
//             .map(|e|->Result<Bytes,PayloadError>{
//                 match e {
//                     Ok(e)=>return Ok(e),
//                     Err(e)=>{
//                         tracing::error!("{:?}",e);
//                         return Ok(Bytes::from(format!("{:?}",e)))
//                     }
//                 }
//             })
//         )
//         ;
//     let b=res.await;
//     println!("{:?}",String::from_utf8(b.unwrap().body().await.unwrap().to_vec()));
//     Ok(WebJSONResult::new(json!({
//
//     })))
// }

//curl  -X POST --data 'xxxxxxxxxxxxx' http://localhost:8080/multipart2
// #[post("/multipart2")]
// pub(crate) async fn multipart2(mut payload1: Multipart) ->Result<WebJSONResult,WebHandError> {
//     let (tx, mut rx) = tokio::sync::broadcast::channel::<String>(100);
//     tokio::task::spawn( async move {
//         let stream = async_stream::stream! {
//             while let Ok(value) = rx.recv().await {
//                 yield std::io::Result::Ok(value);
//             }
//         };
//         let client = Client::new();
//         let builder = client.get("http://httpbin.org/get")
//             .body(reqwest::Body::wrap_stream(stream));
//         let res = builder.send().await.unwrap().text().await.unwrap();
//         println!("{}",res);
//     });
//     while let Ok(Some(mut field)) = payload1.try_next().await {
//         while let Some(chunk) = field.next().await {
//             let data = chunk.unwrap();
//             unsafe {
//                 tx.send(String::from_utf8_unchecked(data.to_vec())).unwrap();
//             }
//         }
//     }
//     Ok(WebJSONResult::new(json!({
//
//     })))
// }
