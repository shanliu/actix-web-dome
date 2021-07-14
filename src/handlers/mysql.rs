use actix_web::{get, web};

use serde::{Deserialize};
use actix_web::Result;
use serde_json::json;
use super::{WebHandError, WebJSONResult};
use super::AppState;


#[derive(Deserialize)]
pub struct AuthRequest {
    id: u64
}
//curl  http://localhost:8080/db?id=1
#[get("/db")]
pub(crate) async fn index<'a>(data: web::Data<AppState<'a>>,web::Query(info):web::Query<AuthRequest>) -> Result<WebJSONResult,WebHandError> {
    let user=data.context.users.find_by_id(info.id as u32).await?;
    Ok(WebJSONResult::new(json!({
         "cat":format!("Hello {}!",user.show_name()),
        "id":info.id
    })))
}
#[get("/db1")]
pub(crate) async fn index1<'a>(data: web::Data<AppState<'a>>,web::Query(info):web::Query<AuthRequest>) -> Result<WebJSONResult,WebHandError> {
    let user=data.context.users.get_user_by_id(info.id.to_string().as_str()).await?;
    Ok(WebJSONResult::new(json!({
         "cat":format!("Hello {}!",user.show_name()),
        "id":info.id
    })))
}
#[get("/db2")]
pub(crate) async fn index2<'a>(data: web::Data<AppState<'a>>,web::Query(_info):web::Query<AuthRequest>) -> Result<WebJSONResult,WebHandError> {
    let user=data.context.users.public_user_fn();
    Ok(WebJSONResult::new(json!({
        "test":user
    })))
}