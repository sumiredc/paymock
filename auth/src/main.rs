mod usecase;

use crate::usecase::login::LoginUseCase;
use actix_cors::Cors;
use actix_web::{App, HttpResponse, HttpServer, Responder, get, http::header, post};
use std::io::Result;

#[actix_web::main]
async fn main() -> Result<()> {
    let addr = format!("{}:{}", "0.0.0.0", "8000"); // TODO: env
    println!("🚀 Auth service listening on http://{}", addr);

    HttpServer::new(|| {
        App::new()
            .wrap(
                Cors::default()
                    .allowed_origin("http://localhost:5173") // TODO: env
                    .allowed_methods(vec!["GET", "POST"])
                    .allowed_headers(vec![header::AUTHORIZATION, header::CONTENT_TYPE])
                    .supports_credentials()
                    .max_age(3600),
            )
            .service(verify)
            .service(create)
            .service(refresh)
    })
    .bind(addr)?
    .run()
    .await
}

#[get("/verify")]
async fn verify() -> impl Responder {
    // TODO
    HttpResponse::Ok()
}

#[post("/login")]
async fn create() -> impl Responder {
    let uc = LoginUseCase::default();

    match uc.execute() {
        Ok(res) => HttpResponse::Ok().json(res),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

#[get("/refresh")]
async fn refresh() -> impl Responder {
    // TODO
    HttpResponse::Ok()
}
