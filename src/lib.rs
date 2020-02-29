#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;
#[macro_use] extern crate rocket_contrib;
#[macro_use] extern crate serde_derive; // json 序列化反序列化宏

#[macro_use]
extern crate diesel;
extern crate dotenv;

use rocket_contrib::databases;

// 数据库
#[database("pg_sql")]
pub struct PgDbConn(databases::diesel::PgConnection);

pub mod schema;
pub mod models;
mod app;
//mod fairings;

// 捕获器 catch
// 附加 附加数据库 fairing 等
// 路由
// 请求保护

// 加载路由
pub fn mount() -> rocket::Rocket {
    rocket::ignite()
        .register(catchers![app::not_found, app::server_error])
        .attach(PgDbConn::fairing()) // 挂载数据库
//        .attach(fairings::ResponseJson::default())
        .mount("/v1/author", routes![app::v1::author::info])
//    rocket::ignite().mount("/", routes![app::v1::author::info])
}