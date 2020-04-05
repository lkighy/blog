use actix_web::{web, App, HttpResponse, HttpServer};
mod blog::app::v1;

// pub fn route(cfg: &mut web::ServiceConfig) {
//     cfg.service(
//         web::service(v1::index::index)
//     )
// }

pub fn index(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::service(v1::index::index)
    );
}