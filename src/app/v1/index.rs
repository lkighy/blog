use actix_web::{HttpResponse, Responder};

use actix_web::get;

#[get("/")]
pub async fn index() -> impl Responder {
    HttpResponse::Ok().body("hello")
}

