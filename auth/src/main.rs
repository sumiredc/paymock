mod handler;
mod usecase;

use actix_cors::Cors;
use actix_web::{App, HttpServer, http::header, web::Data};
use redis::Client;
use std::io::Result;

#[actix_web::main]
async fn main() -> Result<()> {
    let addr = format!("{}:{}", "0.0.0.0", "8000"); // TODO: env
    println!("🚀 Auth service listening on http://{}", addr);

    let client = Client::open("redis://authstore/").expect("Failed to create Redis client");

    HttpServer::new(move || {
        App::new()
            .app_data(Data::new(client.clone()))
            .wrap(
                Cors::default()
                    .allowed_origin("http://localhost:5173") // TODO: env
                    .allowed_methods(vec!["GET", "POST"])
                    .allowed_headers(vec![header::AUTHORIZATION, header::CONTENT_TYPE])
                    .supports_credentials()
                    .max_age(3600),
            )
            .service(handler::verify)
            .service(handler::login)
            .service(handler::refresh)
    })
    .bind(addr)?
    .run()
    .await
}
