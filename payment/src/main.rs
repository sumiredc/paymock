mod handler;
mod usecase;

use actix_cors::Cors;
use actix_web::{App, HttpServer, http::header};
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
                    .allowed_headers(vec![header::CONTENT_TYPE])
                    .supports_credentials()
                    .max_age(3600),
            )
            .service(handler::send)
    })
    .bind(addr)?
    .run()
    .await
}
