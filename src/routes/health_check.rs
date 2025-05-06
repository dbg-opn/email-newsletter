use actix_web::HttpResponse;

// explicitly returning HttpResponse instead of impl Response
pub async fn health_check() -> HttpResponse {
    HttpResponse::Ok().finish()
}
