use actix_web::{HttpRequest, HttpResponse, Responder};

pub async fn health_check(req: HttpRequest) -> impl Responder {
    HttpResponse::Ok().finish()
}
