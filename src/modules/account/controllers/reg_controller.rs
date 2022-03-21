
use actix_web::{get, error, web, Error, HttpResponse, Result};
use crate::appconfig::appconfig::AppConfig;


#[get("/account/reg")]
pub async fn reg(tmpl: web::Data<tera::Tera>, conf: web::Data<AppConfig>) -> Result<HttpResponse, Error>{
    let s = tmpl.render("account/reg.html", &tera::Context::new())
        .map_err(|_| error::ErrorInternalServerError("Termplate error"));

    log::info!("The site name: {}", conf.site_name.to_owned());
    Ok(HttpResponse::Ok().content_type("text/html").body(s.unwrap()))
}

