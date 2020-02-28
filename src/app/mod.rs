use rocket_contrib::json::JsonValue;
use rocket::Request;
use serde::Serialize;

pub mod v1;

#[derive(Serialize)]
pub struct FormatJson<T: Serialize> {
    code: usize,
    msg: String,
    data: T,
}

#[catch(404)]
pub fn not_found(_req: &Request) -> JsonValue {
    json!({
        "code": 404,
        "msg": "not found"
    })
}

#[catch(500)]
pub fn server_error(_req: &Request) -> JsonValue {
    json!({
        "code": 500,
        "msg": "server error"
    })
}