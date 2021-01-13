use actix_web::{App, middleware, HttpServer, web, guard, HttpResponse,error};
use sqlx::{MySql, pool::PoolOptions, ConnectOptions};
use actix_redis::RedisActor;
use std::env;
use dotenv::dotenv;
use crate::handlers::WebData;
use crate::middlewares::user_check::CheckLogin;
use log::LevelFilter;
use sqlx::mysql::MySqlConnectOptions;
use std::str::FromStr;
use tracing_subscriber::EnvFilter;
use actix_session::CookieSession;

mod handlers;
mod models;
mod services;
mod middlewares;
mod utils;

#[actix_web::main]
async fn main() -> futures::io::Result<()> {
    dotenv().ok();


    let dir="logs";
    let file_appender = tracing_appender::rolling::hourly(dir, "app.log");
    let (non_blocking, _guard) = tracing_appender::non_blocking(file_appender);
    tracing_subscriber::fmt()
        //.compact()//是否隐藏参数
        .pretty()
        .with_writer(non_blocking)
        .with_writer(std::io::stdout)
        .with_max_level(tracing::Level::TRACE)
        //.with_env_filter(EnvFilter::from_default_env().add_directive("echo=trace".parse()?))//手动分开配置方式
        //.with_env_filter("async_fn=trace")//格式 模块:最大等级 mod:level
        .try_init().unwrap();
    //输出格式 span{args=3}:span{args=3}: mod::mod: message



    let database_url = env::var("DATABASE_URL").unwrap();
    let mut option =MySqlConnectOptions::from_str(&database_url)
        .unwrap();
    option.log_statements(LevelFilter::Trace);
    let pool = PoolOptions::<MySql>::new()
        .max_connections(5)
        .connect_with(
        option.to_owned()
        )
        .await.unwrap();
    let redis_url = env::var("REDIS_URL").unwrap();
    let redis_addr = RedisActor::start(&redis_url);
    let host = env::var("HOST").unwrap();
    let port = env::var("PORT").unwrap();

    let json_config = web::JsonConfig::default()
        .limit(4096)
        .error_handler(|err, _req| {
            // create custom error response
            error::InternalError::from_response(err, HttpResponse::Conflict().finish()).into()
        });



    HttpServer::new(move||{
        App::new()
            .wrap(middleware::Logger::default())
            .wrap(CheckLogin)
            .wrap(CookieSession::signed(&[0; 32]).secure(false))
            .app_data(json_config.clone())
            .data(WebData {
                app_name: String::from("Actix-web"),
                db:pool.clone(),
                redis:redis_addr.clone()
            })
            .service(handlers::inoutput::index)
            .service(handlers::inoutput::query_get)
            .service(handlers::inoutput::get)
            .service(handlers::inoutput::from)
            .service(handlers::inoutput::payload)
            .service(handlers::inoutput::json)
            .service(handlers::inoutput::path)
            .service(handlers::upload::multipart_save)
            .service(handlers::upload::multipart_page)
            .service(handlers::ws::index)
            .service(web::resource("/ruler/{path_url}")
                 .name("path_name") // <- set resource name, then it could be used in `url_for`
                 .guard(guard::Any(guard::Get())//过滤
                 //   .and(guard::Header("Content-Type", "plain/text"))
                    .or(guard::Post())
                 )
                 .to(handlers::inoutput::ruler))
            .service(handlers::inoutput::session)
            .service(handlers::inoutput::cookie)
            .service(handlers::inoutput::multipart1)
            .service(handlers::inoutput::multipart2)
            .service(handlers::mysql::index)
            .service(handlers::redis::index)
            .service(actix_files::Files::new("/static", "./static").show_files_listing())
            .external_resource("baidu", "https://baidu.com/s/{key}")
            .default_service(web::resource("").route(web::get().to(handlers::p404)))
    })
    .bind(format!("{}:{}",host,port))?
    .run()
    .await
}