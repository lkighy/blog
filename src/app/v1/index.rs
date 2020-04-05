use actix_web::{web};

use actix_web::get;

#[get("/")]
pub async fn index() -> String {
    // HttpResponse::Ok().body("hello")
    String::from("hello")
}

// this function could be located in different module
pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(index);
}

