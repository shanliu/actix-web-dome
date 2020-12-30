use actix_web::{ App, HttpServer};
mod handler;

struct SiteSetting {
    app_name: String
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {

    HttpServer::new(|| {
        App::new()
            .data(SiteSetting {
                app_name: String::from("Actix-web"),
            })
            .service(handler::group1::index)

    })
        .bind("127.0.0.1:8080")?
        .run()
        .await
}