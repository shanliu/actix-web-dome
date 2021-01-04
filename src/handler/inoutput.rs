use actix_web::{get, web, HttpResponse};
use crate::SiteSetting;
use serde::{Deserialize, Serialize};
use actix_web::{error, Result,error::ResponseError};
use derive_more::{Display, Error};
use serde_json::json;
use sqlx::mysql::MySqlRow;
use sqlx::{Row, Error};
use actix_redis::Command;
use redis_async::{resp::RespValue, resp_array};
use serde::export::Formatter;


#[derive(Debug, Display, Error)]
#[display(fmt = "my error: {}", name)]
pub struct WebHandError{
    name: String,
}
impl ResponseError for WebHandError{

}
impl From<sqlx::Error> for WebHandError{
    fn from(err: sqlx::Error) -> Self {
        let b=err.as_database_error().unwrap().message().to_string();
        return WebHandError{
            name:b
        }
    }
}


#[derive(Serialize, Deserialize)]
struct MyObj {
    name: String,
}
#[derive(Deserialize)]
pub struct AuthRequest {
   id: u64,
   name: String,
}

#[derive(sqlx::FromRow)]
struct Account {
    id: i32,
    name: String
}


#[get("/")]
pub(crate) async fn index(web::Query(info):web::Query<AuthRequest>,data: web::Data<SiteSetting>) -> Result<HttpResponse,WebHandError> {
    let app_name = &data.app_name;

    let mut stream = sqlx::query_as::<_, Account>(
        "
SELECT 'aa' as name, 1 as id
        "
    )
        .fetch_one(&data.db).await?;


   let b=format!("{}-{}",stream.name,stream.id);

    let row = sqlx::query_as("SELECT Fid,Frule_data from t_pm_valid_data_rule where Fid>=?")
        .bind(info.id as u32)
        .fetch_one(&data.db).await;
    let brow:(u32,String)=row.map_err(|e|{
        match e {
            Error::Database(err) =>{
                let b=err.message().to_string();
                WebHandError{name:b}
            },
            Error::RowNotFound=>{
                return WebHandError{name:"无记录".to_string()};
            }
            x@_ => {
                let b:String=format!("{:?}", x);
                return WebHandError{name:b};
            },
        }
    })?;

    Ok(HttpResponse::Ok().json(MyObj {
        name: format!("Hello {} {} {}!", brow.0,brow.1,b)
    }))
}


#[get("/show/{cat}/{id}")]
pub(crate) async fn user_detail(web::Path((cat,id)):web::Path<(String, i32)>,data: web::Data<SiteSetting>) -> Result<HttpResponse,WebHandError> {

    let one = data.redis.send(Command(resp_array!["SET", "mydomain:one", "info.one"]));
    let b=one.await;


    Ok(HttpResponse::Ok().json(json!({
        "cat":cat,
        "id":id
    })))
}
