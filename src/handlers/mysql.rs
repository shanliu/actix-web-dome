use actix_web::{get, web, HttpResponse};

use serde::{Deserialize};
use actix_web::Result;
use serde_json::json;
use crate::handlers::WebHandError;
use crate::AppState;


#[derive(Deserialize)]
pub struct AuthRequest {
    id: u64
}
//curl  http://localhost:8080/db?id=1
#[get("/db")]
pub(crate) async fn index<'a>(web::Query(info):web::Query<AuthRequest>,data: web::Data<AppState<'a>>) -> Result<HttpResponse,WebHandError> {
    let user=data.context.users.find_by_id(info.id as u32).await?;
    Ok(HttpResponse::Ok().json(json!({
       "cat":format!("Hello {}!",user.show_name()),
        "id":info.id
    })))
}