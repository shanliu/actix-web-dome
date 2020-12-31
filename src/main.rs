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
mod handler;

struct SiteSetting {
    app_name: String
}

async fn p404() -> Result<fs::NamedFile> {
    Ok(fs::NamedFile::open("static/404.html")?.set_status_code(StatusCode::NOT_FOUND))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "actix_web=info");
    HttpServer::new(|| {
        App::new()
            .wrap(middleware::Logger::default())
            .data(SiteSetting {
                app_name: String::from("Actix-web"),
            })
            .service(
            handler::inoutput::index
            )
            // .service(
            // handler::inoutput::user_detail
            // )
            .service(
            web::resource("/test/{c}")
                     .name("foo") // <- set resource name, then it could be used in `url_for`
                     .guard(guard::Post())
                     .to(|| HttpResponse::Ok().body("dddddddddddddd"))
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
        .bind("127.0.0.1:80")?
        .run()
        .await
}