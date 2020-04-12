use actix_web::{App, HttpServer};
mod conf;
mod app;

use actix_web::middleware;

use mongodb::{Client};
use mongodb::options::{ClientOptions, StreamAddress};
use actix_redis::{RedisActor};

#[actix_rt::main]
async fn main() -> std::io::Result<()> {

    HttpServer::new(|| {
        let mongo = {
            let options = ClientOptions::builder()
                .hosts(vec![
                    StreamAddress {
                        hostname: conf::CONF.mongodb_hostname.into(),
                        port: Some(conf::CONF.mongodb_port),
                    }
                ])
                .build();
            let client = Client::with_options(options).expect("连接mongodb 失败");
            client.database(conf::CONF.mongodb_name)
        };
        let redis_addr = RedisActor::start(conf::CONF.redis_addr);
        App::new()
            .data(mongo)
            .data(redis_addr)
            .warp(middleware::Logger::default())
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