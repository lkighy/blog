use actix_web::web;
use serde::{Serialize};

#[derive(Serialize)]
struct ResultData<T> {
    code: u32,
    msg: String,
    data: T,
}


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
