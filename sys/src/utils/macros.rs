use actix_web::web;
use serde::{Serialize};

#[derive(Serialize)]
struct ResultData<T> {
    code: u32,
    msg: String,
    data: T,
}

macro_rules! ResultJSON {
    ($code:tt) => {
        web::Json(ResultData {
            code: $code,
            msg: String::new(),
            data: String::new(),
        })
    };
    ($code:tt, $msg:tt) => {
        web::Json(ResultData {
            code:$code,
            msg:format!("{:?}", $msg),
            data: String::new(),
        })
    };
    ($code:tt, $msg:tt, $data:tt) => {
        web::Json(ResultData {
            code: format!("{:?}", $code),
            msg: $msg,
        })
    };
}

// 建立一个宏，用于简略查询
