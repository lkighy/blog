use actix_web::{App, HttpServer};
use blog::index;

#[actix_rt::main]
fn main() -> std::io::Result<()> {
    // println!("{}", conf::conf::conf.redis_connection);
    HttpServer::new(|| {
        App::new()
            .configure(index)
    })
    .bind("127.0.0.1:8088")?
    .run()
    .await
}
