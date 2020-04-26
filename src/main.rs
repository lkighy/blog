#[macro_use]
extern crate dotenv_codegen;

use actix_web::{App, HttpServer};
use actix_web::middleware::Logger;

use actix_redis::{RedisActor, RedisSession};

use mongodb::{Client};
use mongodb::options::{ClientOptions, StreamAddress};

// mod conf;
mod app;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    // 转换 byte
    let app_addr = format!("{}:{}", dotenv!("APP_HOSTNAME"), dotenv!("APP_PORT"));
    HttpServer::new(|| {
        let mongodb_port = format!("{}", dotenv!("MONGODB_PORT"));
        let mongodb_hostname = format!("{}", dotenv!("MONGODB_HOSTNAME"));
        let mongodb_name = format!("{}", dotenv!("MONGODB_NAME"));
        let mongo = {
            let options = ClientOptions::builder()
                .hosts(vec![
                    StreamAddress {
                        hostname: mongodb_hostname.clone(),
                        port: Some(mongodb_port.parse().expect("mongodb_port 不是数值类型")),
                    }
                ])
                .build();
            let client = Client::with_options(options).expect("连接mongodb 失败");
            client.database(&mongodb_name)
        };
        let redis_addr = dotenv!("REDIS_ADDR");
        let key = format!("{}", dotenv!("SESSION_KEY"));
        let private_key = key.as_bytes();
        let redis = RedisActor::start(redis_addr);
        // 生成随机 key, 这里应该用一个固定的随机 key 代替
        App::new()
            .wrap(Logger::default())
            .wrap(Logger::new("%a %{User-Agent}i"))
            .wrap(RedisSession::new(redis_addr, private_key))
            .data(mongo)
            .data(redis)
            .configure(app::v1::index_config)
            .configure(app::admin::login_config)

    })
    .bind(app_addr)?
    .run()
    .await
}