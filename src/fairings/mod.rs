// 格式化响应请求

use rocket::{Request, Data, Response};
use rocket::fairing::{Fairing, Info, Kind};
use rocket::http::{Method, ContentType, Status};
use rocket_contrib::json::JsonValue;

#[derive(Default)]
pub struct ResponseJson {
    code: usize,
    data: JsonValue,
    msg: String,
}


// Fairing 将用在数据统计上而不再进行数据上的操作
impl Fairing for ResponseJson {
    fn info(&self) -> Info {
        Info {
            name: "return json data",
            kind: Kind::Response
        }
    }

    fn on_response(&self, request: &Request, response: &mut Response) {
        // 正常内容不做处理， 但是会对 404 500 这写内容进行修改？

    }
}