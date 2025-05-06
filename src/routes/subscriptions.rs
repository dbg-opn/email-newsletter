use crate::FormData;
use actix_web::{HttpResponse, web};

// At the start always return 200 OK
pub async fn subscribe(_form: web::Form<FormData>) -> HttpResponse {
    HttpResponse::Ok().finish()
}
