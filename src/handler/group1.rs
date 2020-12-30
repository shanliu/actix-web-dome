use actix_web::{get, web};
use crate::SiteSetting;

#[get("/")]
pub(crate) async fn index(data: web::Data<SiteSetting>) -> String {
    let app_name = &data.app_name; // <- get app_name

    format!("Hello {}!", app_name) // <- response with app_name
}



