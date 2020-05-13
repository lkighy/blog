use actix_web::{post, get, web, HttpRequest, Responder};

extern crate redis;
use redis::Connection;
use std::net::SocketAddr;
use self::redis::Commands;
use std::sync::Mutex;

use serde::{Serialize};

use mongodb::Database;
use mongodb::options::FindOptions;
use bson::{Bson, doc};

use crate::utils::{tools, smtp};

#[derive(Serialize)]
struct ResultData<T> {
    code: u32,
    msg: String,
    data: T,
}

#[derive(Deserialize)]
struct send_ckm {
    email: String,
    skm: String,
}

/// 发送验证码到邮箱
#[get("/send-ckm")]
pub async fn send_ckm(
    req: HttpRequest,
    data: web::Json<send_ckm>,
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
    // todo 1: 查询该 ip 是否已被拉黑
    match con.sismember("blacklist", &ip) {
        Ok(black) =>  {
            if black {
                return web::Json(ResultData {
                    code: 306,
                    msg: String::new(),
                    data: String::new(),
                });
            }

        },
        Err(e) => return web::Json(ResultData {
            code: 306,
            msg: format!("{:?}", e),
            data: String::new(),
        }),
    };

    let mut ckm_arr: Vec<u8> = vec![];

    // todo 2: 生成随机数
    let ckm = tools::generate_verify();

    // 从数据库中得到数据，账号密码，通过哪个代理发送，以及代理的端口,最后就是模板了喔
    let filter = doc!{"email", data.email};
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