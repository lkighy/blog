#![feature(proc_macro_hygiene, decl_macro)]
//use rocket_contrib::json::Json;

#[post("/")]
pub fn info() -> String {
  String::from("this Author")
}

