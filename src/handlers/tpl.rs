use actix_web::{get, web, HttpResponse};
use crate::handlers::WebHandError;


#[get("/tpl")]
pub(crate) async fn index( tmpl: web::Data<tera::Tera>) ->Result<HttpResponse,WebHandError> {
        let mut ctx = tera::Context::new();
        ctx.insert("name", &"ddd".to_owned());
        ctx.insert("text", &"Welcome!".to_owned());
        let s = tmpl.render("err.html", &ctx)?;
    Ok(HttpResponse::Ok().content_type("text/html").body(s))
}
