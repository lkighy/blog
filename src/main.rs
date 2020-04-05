use actix_web::{App, HttpServer};
// use blog::index;
mod conf;
mod app;

// #[actix_rt::main]
// async fn main() -> std::io::Result<()> {
//     // println!("{}", conf::conf::conf.redis_connection);
//     HttpServer::new(|| {
//         App::new().service(
//             web.route(web::get().to(app::v1::index::index)),
//         )
//     })
//         .bind(format!("{}:{}", conf::conf::conf.app_hostname,
//                       conf::conf::conf.app_port))?
//         .run()
//         .await
// }
#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new().configure(app::v1::index::config)
    })
    .bind(
        format!("{}:{}", conf::conf::CONF.app_hostname,
            conf::conf::CONF.app_port
        )
    )?
    .run()
    .await
}