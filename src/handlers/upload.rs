use actix_web::{get, post, web, HttpResponse};
use actix_multipart::Multipart;
use super::{WebJSONResult, WebHandError};
use serde_json::json;
use std::io::Write;
use futures::{StreamExt, TryStreamExt};
use super::{ProgramError};
use std::path::{Path};




#[get("/multipart")]
pub(crate) async fn multipart_page() ->HttpResponse {
    let html = r#"<html>
        <head><title>Upload Test</title></head>
        <body>
            <form target="/multipart" method="post" enctype="multipart/form-data">
                <input type="file" multiple name="file"/>
                <button type="submit">Submit</button>
            </form>
        </body>
    </html>"#;
    HttpResponse::Ok().body(html)
}
#[post("/multipart")]
pub(crate) async fn multipart_save(mut payload: Multipart) -> Result<WebJSONResult, WebHandError> {
    // iterate over multipart stream
    while let Ok(Some(mut field)) = payload.try_next().await {
        let content_type = field.content_disposition().unwrap();
        let filename = content_type.get_filename().unwrap();
        tracing::info!(%filename);
        let filename=String::from(html_escape::decode_html_entities(filename));
        let filepath = format!("./logs/{}", &filename);
        // File::create is blocking operation, use threadpool
        let _filepath=filepath.clone();
        let mut f = web::block(|| std::fs::File::create(_filepath))
            .await
            .unwrap()
            .map_err(|e| ProgramError::Io {
                source: e,
                path: Path::new(filepath.clone().as_str()).to_path_buf(),
            })
            ?;
        let mut bad =None;
        // Field in turn is stream of *Bytes* object
        while let Some(chunk) = field.next().await {
            if chunk.is_err() {
                tracing::error!("{:?}",&chunk);
                bad=Some(chunk);
                break;
            }
            //文件从网络得到,并在这里开始写
            let data = chunk.unwrap();
            //tracing::info!("info {}",data.len());
            let tt=move || -> Result<std::fs::File, std::io::Error>{
                //f 的所有权被移入到闭包
                //完成时把f所有权返回
                let n:Result<(),std::io::Error>=f.write_all(&data);
                n.map(|_|{
                    f//Result的内部类型转换快捷方式,错误时候不转,如果错误时返回包装Err
                } )
                // if n.is_err() {
                //     return n.map(|e|f);
                // }
                // return Result::Ok(f);
            };
            // filesystem operations are blocking, we have to use threadpool
            f = web::block(tt).await.unwrap().unwrap();//重新在这里得到f的所有权
        }
        if bad.is_some() {
            let _bad=bad.unwrap();
            return Err(WebHandError::new(format!("{:?}",_bad)));
        }
    }
    Ok(WebJSONResult::new(json!({
        "a":"b"
    })))
}