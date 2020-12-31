use actix_web::{get, web, HttpResponse};
use crate::SiteSetting;
use serde::{Deserialize, Serialize};

#[get("/")]
pub(crate) async fn index(data: web::Data<SiteSetting>) -> String {
    let app_name = &data.app_name;
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
