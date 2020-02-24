#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;

mod app;

// 加载路由
pub fn mount() -> rocket::Rocket {
    let r = rocket::ignite();
    r.mount("/blog/author", route![app::blog::author::info]);
//    rocket::ignite().mount("/", routes![app::blog::author::info])
    r
}