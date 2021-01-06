use actix_web::{
    App,
    middleware,
    HttpServer,
    web
};
use sqlx::{
    MySql,
    pool::PoolOptions
};
use actix_redis::RedisActor;
use std::env;
use dotenv::dotenv;
use crate::handlers::WebData;
use crate::middlewares::user_check::CheckLogin;

mod handlers;
mod models;
mod services;
mod middlewares;

#[actix_web::main]
async fn main() -> futures::io::Result<()> {
    dotenv().ok();
    let rust_log = env::var("RUST_LOG").unwrap();
    std::env::set_var("RUST_LOG", rust_log);
    let database_url = env::var("DATABASE_URL").unwrap();
    let pool = PoolOptions::<MySql>::new()
        .max_connections(5)
        .connect(&database_url).await.unwrap();
    let redis_url = env::var("REDIS_URL").unwrap();
    let redis_addr = RedisActor::start(&redis_url);
    let host = env::var("HOST").unwrap();
    let port = env::var("PORT").unwrap();
    HttpServer::new(move||{
        App::new()
            .wrap(middleware::Logger::default())
            .wrap(CheckLogin)
            .data(WebData {
                app_name: String::from("Actix-web"),
                db:pool.clone(),
                redis:redis_addr.clone()
            })
            .service(handlers::inoutput::index)
            .service(handlers::mysql::index)
            .service(handlers::redis::index)
            .service(actix_files::Files::new("/static", "./static").show_files_listing())
            .default_service(web::resource("").route(web::get().to(handlers::p404)))
    })
    .bind(format!("{}:{}",host,port))?
    .run()
    .await
}