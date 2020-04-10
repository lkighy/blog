use actix_web::{App, HttpServer};
mod conf;
mod app;


#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .configure(app::v1::index_config)
    })
    .bind(
        format!("{}:{}", conf::CONF.app_hostname,
            conf::CONF.app_port
        )
    )?
    .run()
    .await
}