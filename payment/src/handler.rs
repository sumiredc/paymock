use crate::usecase::send::SendUseCase;
use actix_web::{HttpRequest, HttpResponse, Responder, http::header::HeaderMap, post};

#[post("/send")]
pub async fn send(req: HttpRequest) -> impl Responder {
    let uc = SendUseCase::new();

    match parse_user_id(req.headers()) {
        Ok(user_id) => match uc.execute(user_id) {
            Ok(res) => HttpResponse::Ok().json(res),
            Err(_) => HttpResponse::InternalServerError().finish(),
        },
        Err(_) => HttpResponse::BadRequest().finish(),
    }
}

// TODO: 共通化する
fn parse_user_id(headers: &HeaderMap) -> Result<String, ()> {
    match headers.get("X-Authenticated-User-ID") {
        Some(val) => match val.to_str() {
            Ok(user_id) => Ok(user_id.to_string()),
            Err(_) => Err(()),
        },
        None => Err(()),
    }
}
