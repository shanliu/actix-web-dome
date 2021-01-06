use actix_web::{get, web, HttpResponse};
use crate::WebData;
use serde::{Deserialize};
use actix_web::Result;
use serde_json::json;
use crate::handlers::WebHandError;
use crate::services::account::AccountSerice;


#[derive(Deserialize)]
pub struct AuthRequest {
    id: u64
}

#[get("/db")]
pub(crate) async fn index(web::Query(info):web::Query<AuthRequest>,data: web::Data<WebData>) -> Result<HttpResponse,WebHandError> {
    let user=AccountSerice::new(&data.db).find_by_id(info.id as u32).await?;
    Ok(HttpResponse::Ok().json(json!({
        "cat":format!("Hello {}!",user.show_name()),
        "id":info.id
    })))
}