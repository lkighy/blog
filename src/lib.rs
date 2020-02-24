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

// 加载路由
pub fn mount() -> rocket::Rocket {
    rocket::ignite()
        .attach(PgDbConn::fairing())
        .mount("/v1/author", routes![app::v1::author::info])
//    rocket::ignite().mount("/", routes![app::v1::author::info])
}