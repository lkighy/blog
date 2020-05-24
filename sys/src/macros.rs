use actix_web::web;
use serde::{Serialize};

#[derive(Serialize)]
pub struct ResultData<T> {
    pub code: u32,
    pub msg: String,
    pub data: T,
}

#[macro_export]
macro_rules! handle_error_json {
 (Result, $result:tt) => {
    match $result {
        Ok(result) => result,
        Err(e) => {
            return web::Json(ResultData {
                code: 500,
                msg: format!("{:?}", e),
                data: String::new(),
            })
        },
    }
 };
 (Result, $result:tt, $code: tt) => {
    match $result {
        Ok(result) => result,
        Err(e) => {
            return web::Json(ReusltData {
                code: $code,
                msg: format!("{:?}", e),
                data: String::new(),
            })
        }
    }
 };
 (Option, $option:tt) => {
     match $option {
        Some(option) => option,
        None => return web::Json(ResultData {
            code: 500,
            msg: String::from("system error: ip is None"),
            data: String::new(),
        })
     }
 };
 (Option, $option:tt, $code:tt) => {
     match $option {
        Some(option) => option,
        None => return web::Json(ResultData {
            code: $code,
            msg: String::from("system error: ip is None"),
            data: String::new(),
        })
     }
 };
}
// 建立一个宏，用于简略查询
