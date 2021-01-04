use actix_web::{
    App,
    middleware,
    HttpServer,
    Result,
    web,
    guard,
    HttpResponse,
    http::{
        StatusCode
}};

use actix_files as fs;
use sqlx::{ MySql, Pool};
use sqlx::pool::PoolOptions;
use actix_redis::RedisActor;
use actix::Addr;


mod handler;

struct SiteSetting {
    app_name: String,
    db:Pool<MySql>,
    redis:Addr<RedisActor>
}

async fn p404() -> Result<fs::NamedFile> {
    Ok(fs::NamedFile::open("static/404.html")?.set_status_code(StatusCode::NOT_FOUND))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let pool = PoolOptions::<MySql>::new()
        .max_connections(5)
        .connect("mysql://root:@127.0.0.1/test").await.unwrap();
    let redis_addr = RedisActor::start("127.0.0.1:6379");
    std::env::set_var("RUST_LOG", "actix_web=info");
    HttpServer::new(move|| {
        App::new()
            .wrap(middleware::Logger::default())
            .data(SiteSetting {
                app_name: String::from("Actix-web"),
                db:pool.clone(),
                redis:redis_addr.clone()
            })
            .service(
            handler::inoutput::index
            )
            .service(
            handler::inoutput::user_detail
            )
            .default_service(
                web::resource("")
                    .route(web::get().to(p404))
                    .route(
                        web::route()
                            .guard(guard::Not(guard::Get()))
                            .to(HttpResponse::MethodNotAllowed)
                    )
            )

    })
        .bind("127.0.0.1:8080")?
        .run()
        .await
}