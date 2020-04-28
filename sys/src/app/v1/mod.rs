mod index;

use actix_web::{web};

pub fn index_config(cfg: &mut web::ServiceConfig) {
    cfg.service(index::index);
}
