use crate::usecase::{login::LoginUseCase, verify::VerifyUseCase};
use actix_web::{HttpResponse, Responder, get, post, web::Header};
use actix_web_httpauth::headers::authorization::{Authorization, Bearer};

#[get("/verify")]
pub async fn verify(header: Header<Authorization<Bearer>>) -> impl Responder {
    let auth = header.into_inner();
    let bearer = auth.into_scheme();
    let token = bearer.token();

    let uc = VerifyUseCase::new();

    match uc.execute(token) {
        Ok(res) => HttpResponse::Ok()
            .insert_header(("X-User-Id", res.user_id))
            .finish(),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

#[post("/login")]
pub async fn login() -> impl Responder {
    let uc = LoginUseCase::new();

    match uc.execute() {
        Ok(res) => HttpResponse::Ok().json(res),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

#[get("/refresh")]
pub async fn refresh() -> impl Responder {
    // TODO
    HttpResponse::Ok()
}
