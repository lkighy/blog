mod login;

use actix_web::{web};

pub fn login_config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/login")
            .service(login::send_ckm)
            .service(login::verify_ckm)
    );
}
