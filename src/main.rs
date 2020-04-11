use actix_web::{App, HttpServer};
mod conf;
mod app;

use mongodb::{Client};
use mongodb::options::{ClientOptions, StreamAddress};

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    let Mongo = {
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

    HttpServer::new(|Mongo| {
        App::new()
            .data(Mongo.clone())
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