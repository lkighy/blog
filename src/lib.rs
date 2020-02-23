#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;

#[get("/")]
fn index() -> &'static str {
    "hello, world!"
}

// 加载路由
pub fn mount() -> rocket::Rocket {
    rocket::ignite().mount("/", routes![index])
}