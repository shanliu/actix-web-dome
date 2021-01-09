use actix_web::{get, post, web, HttpResponse};
use actix_multipart::Multipart;
use crate::handlers::{WebJSONResult, WebHandError};
use serde_json::json;
use std::io::Write;
use futures::{StreamExt, TryStreamExt};


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

        let filepath = format!("./tmp/{}", &filename);

        // File::create is blocking operation, use threadpool
        let mut f = web::block(|| std::fs::File::create(filepath))
            .await
            .unwrap();

        // Field in turn is stream of *Bytes* object
        while let Some(chunk) = field.next().await {
            let data = chunk.unwrap();
            // filesystem operations are blocking, we have to use threadpool
            f = web::block(move || f.write_all(&data).map(|_| f)).await.unwrap();
        }
    }
    Ok(WebJSONResult::new(json!({
        "a":"b"
    })))
}