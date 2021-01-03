use actix_web::{get, web, HttpResponse};
use crate::SiteSetting;
use serde::{Deserialize, Serialize};
use sqlx::mysql::MySqlPoolOptions;


#[get("/")]
pub(crate) async fn index(data: web::Data<SiteSetting>) -> String {
    let app_name = &data.app_name;


    let row: (i64,) = sqlx::query_as("SELECT $1")
        .bind(150_i64)
        .fetch_one(&data.db).await.unwrap();

    assert_eq!(row.0, 150);

    format!("Hello {}!", app_name)
}


#[get("/show/{cat}/{id?}")]
pub(crate) async fn user_detail(web::Path((cat,id)):web::Path<(String, i32)>) -> HttpResponse {
    #[derive(Debug, Serialize, Deserialize)]
    struct MyObj {
        cat: String,
        id: i32,
    }
    HttpResponse::Ok().json(MyObj{
        cat:cat,
        id:id
    })
}
