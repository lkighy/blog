use actix_web::{post, get, web, HttpRequest, Responder};

extern crate redis;
use redis::Connection;
use std::net::SocketAddr;
use self::redis::Commands;
use std::sync::Mutex;

use serde::{Serialize};

use mongodb::Database;
// use mongodb::options::FindOptions;
use bson::{doc, Document};

use crate::utils::{tools};
use mongodb::options::FindOptions;


#[derive(Serialize)]
struct ResultData<T> {
    code: u32,
    msg: String,
    data: T,
}

// #[derive(Deserialize)]
// struct send_ckm {
//     email: String,
//     skm: String,
// }

/// 发送验证码到邮箱
#[get("/send-ckm")]
pub async fn send_ckm(
    req: HttpRequest,
    // data: web::Json<send_ckm>,
    redis_con: web::Data<Mutex<Connection>>,
    db: web::Data<Database>
) -> impl Responder {
    // 获取 ip 地址，
    let addr = match req.peer_addr() {
        Some(addr) => addr,
        None => panic!(),
    };
    let ip = match addr {
        SocketAddr::V4(ipv4) => format!("{:?}", ipv4.ip()),
        SocketAddr::V6(ipv6) => format!("{:?}", ipv6.ip()),
    };

    // 得到 redis conn
    let mut con = redis_con.lock().unwrap();
    // todo 1: 通过 email:ip 查询是否以生成验证码
    // let is_cmk: bool = con.get(format!("{}:{}", email, ip)).expect("hello");
    let is_cmk= con.get(format!("{}:{}", email, ip));
    let is_cmk: bool = handle_error_json!{Resule, is_cmk};

    // todo 2: 查询是否生成验证码的时间是否已经经过 1 分钟 -> 提示一分钟后再发送
    // todo 3: 查询是否以生成三次验证码码 -> 加入临时黑名单
    // match con.sismember("blacklist", &ip) {
    //     Ok(black) =>  {
    //         if black {
    //             return web::Json(ResultData {
    //                 code: 306,
    //                 msg: String::new(),
    //                 data: String::new(),
    //             });
    //         }
    //     },
    //     Err(e) => return web::Json(ResultData {
    //         code: 306,
    //         msg: format!("{:?}", e),
    //         data: String::new(),
    //     }),
    // };

    // todo 2: 生成随机数
    let ckm = tools::generate_verify();

    let coll = db.collection("stmp_info");
    // 从数据库中得到数据，账号密码，通过哪个代理发送，以及代理的端口,最后就是模板了喔
    let filter = doc! {"email": "1003027913@qq.com"};
    let find_options = FindOptions::builder().build();
    let cursor = coll.find(filter, None);
    let cursor = match cursor {
        Ok(cursor) => cursor,
        Err(e) => {
            return web::Json(ResultData {
                code: 306,
                msg: format!("{:?}", e),
                data:String::new(),
            });
        }
    };

    let result = cursor.collect();

    // let coll = db.collection("");
    // let result = coll.insert_one(doc! {"": 1}, None);
    // 提取出数据库的查询操作，单独放到一个模块中
    // let filter = doc!{"email", data.email};
    // 假设我有这么一个宏，接受db,接受 collection以及一个

    // ResultJSON!(200, "", String::from_utf8_lossy(&ckm_arr))

    // 得到邮件模板
    // let collection = db.collection("template");
    // let filter = doc! {"title", "smtp"};
    // let find_options = mongodb::options::FindOneOptions::builder().build();
    // let cursor = collection.find(filter, find_options).expect("查询失败");

    // utils::smtp::data::new()
    web::Json(ResultData {
        code: 200,
        msg: String::new(),
        data: ckm,
    })
}

#[post("/verify-ckm")]
pub async fn verify_ckm() -> String {
    // 获取 ip
    // 查询该 ip 下的验证码是否相同
    String::from("verify-ckm")
}

// redis 关于验证登录这块的参数格式
// 1. 记录 ip
// 2. 记录发送次数，错误次数
// 3. 验证码
// 4. `