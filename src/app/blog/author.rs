#![feature(proc_macro_hygiene, decl_macro)]
use rocket_contrib::json::Json;

[#[derive(Serialize)]]
struct AuthorList {
    name: String,
    email: String,
    birth: String,
    hobby: String,
    introduce: String,
}

#[post("/author/info")]
fn authorList() -> Json<AuthorList> {
    AuthorList{
        name: String::from(""),
        emial: String::from(""),
        birth: String::from(""),
        hobby: String::from(""),
        introduce: String::from(""),
    }
}