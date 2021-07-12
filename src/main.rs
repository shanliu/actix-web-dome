use actix_web::{App, middleware, HttpServer, web, guard, HttpResponse,error};
use sqlx::{MySql, pool::PoolOptions, ConnectOptions};
use actix_redis::RedisActor;
use std::env;
use dotenv::dotenv;
// use crate::middlewares::user_check::CheckLogin;
use log::LevelFilter;
use sqlx::mysql::MySqlConnectOptions;
use std::str::FromStr;
use crate::handlers::AppState;
use std::sync::{Arc};
use crate::daos::{ Dao};
use tera::Tera;
use actix_web::web::Data;
use actix::Addr;
use actix_session::CookieSession;

mod handlers;
mod models;
mod daos;
mod middlewares;
mod utils;

#[actix_web::main]
async fn main() -> futures::io::Result<()> {
    dotenv().ok();

    let log_level = env::var("LOG_LEVEL").unwrap();

  //  let dir = env::var("LOG_DIR").unwrap();
 //   let name = env::var("LOG_NAME").unwrap();
   // let file_appender = tracing_appender::rolling::hourly(dir, name);
  //  let (non_blocking, _guard) = tracing_appender::non_blocking(file_appender);
    tracing_subscriber::fmt()
        //.compact()//是否隐藏参数
       // .pretty()
       // .with_writer(non_blocking)//只能有一个writer
        .with_writer(std::io::stdout)//只能有一个writer
        .with_max_level(tracing::Level::TRACE)
        //.with_env_filter(EnvFilter::from_default_env().add_directive("echo=trace".parse()?))//手动分开配置方式
        // 基于span过滤 target[span{field=value}]=level
        .with_env_filter(log_level)//格式 模块:最大等级 mod:level
        .try_init().unwrap();
    //输出格式 span{args=3}:span{args=3}: mod::mod: message

    // tracing_log::LogTracer::builder().with_max_level(log::LevelFilter::Debug).init().unwrap();
    // let address: std::net::SocketAddr = "127.0.0.1:12201".parse().unwrap();
    // let bg_task = tracing_gelf::Logger::builder().init_tcp(address).unwrap();
    // actix_web::rt::spawn(bg_task);

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

    let db_context = Dao::new(pool.clone()).await;


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
    let webdata=web::Data::new(AppState {
        context:Arc::new(db_context),
        app_name: String::from("Actix-web"),
        db_pool:pool.clone(),
        redis:redis_addr.clone()
    });

    HttpServer::new(move||{

        let tera =
            Tera::new(concat!(env!("CARGO_MANIFEST_DIR"), "/src/templates/**/*")).unwrap();

        App::new()
            .wrap(middleware::Logger::default())
            // .wrap(CheckLogin)
            .wrap(CookieSession::signed(&[0; 32]).secure(false))
            .app_data(Data::new(json_config.clone()))//每个线程独立数据
            .app_data(Data::new(tera))//每个线程独立数据
            .app_data(webdata.clone())//每个线程共享数据
            .service(handlers::inoutput::index)
            .service(handlers::inoutput::usertype)
            .service(handlers::inoutput::query_get)
            .service(handlers::inoutput::get)
            .service(handlers::inoutput::from)
            .service(handlers::inoutput::payload)
            .service(handlers::inoutput::json)
            .service(handlers::inoutput::path)
            .service(handlers::upload::multipart_save)
            .service(handlers::upload::multipart_page)
            .service(handlers::log::log1)
           .service(handlers::ws::index)
            .service(web::resource("/ruler/{path_url}")
                 .name("path_name") // <- set resource name, then it could be used in `url_for`
                 .guard(guard::Any(guard::Get())//过滤
                 //   .and(guard::Header("Content-Type", "plain/text"))
                    .or(guard::Post())
                 )
                 .to(handlers::inoutput::ruler))
            .service(handlers::inoutput::cookie)
            .service(handlers::inoutput::payload1)
            .service(handlers::client::multipart2)
            .service(handlers::mysql::index)
            .service(handlers::mysql::index1)
           .service(handlers::redis::index)
            // .service(handlers::client::multipart1)
            .service(handlers::inoutput::session)
            .service(handlers::tpl::index)
            .service(actix_files::Files::new("/static", "./static").show_files_listing())
            .external_resource("baidu", "https://baidu.com/s/{key}")
            // .default_service(web::resource("").route(web::get().to(handlers::p404)))
    })
    .workers(4)
    .bind(format!("{}:{}",host,port))?
    .run()
    .await
}