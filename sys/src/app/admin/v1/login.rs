use actix_web::{post, get, web, HttpRequest, Responder};

extern crate redis;
use redis::Connection;
use std::net::SocketAddr;
use self::redis::Commands;
use std::sync::Mutex;

use serde::{Serialize};

use rand;
use rand::distributions::{Distribution, Uniform};


use mongodb::Database;
use mongodb::options::FindOptions;
use bson::{Bson, doc};



#[derive(Serialize)]
struct ResultData<T> {
    code: u32,
    msg: String,
    data: T,
}


/// 发送验证码到邮箱
#[get("/send-ckm")]
pub async fn send_ckm(
    req: HttpRequest,
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
    let is_black:bool = match con.sismember("blacklist", &ip) {
        Ok(black) =>  black,
        Err(e) => return web::Json(ResultData {
            code: 306,
            msg: format!("{:?}", e),
            data: String::new(),
        }),
    };
    if is_black {
        return web::Json(ResultData {
            code: 306,
            msg: String::new(),
            data: String::new(),
        });
    }
    let mut ckm_arr: Vec<u8> = vec![];

    // todo 2: 生成随机数
    let step = Uniform::from(48..58);
    for _ in 0..6 {
        ckm_arr.push(step.sample(&mut rand::thread_rng()));
    }
    // let ckm = format!("{}", String::from_utf8_lossy(&ckm_arr));

    // ResultJSON!(200, "", String::from_utf8_lossy(&ckm_arr))

    web::Json(ResultData {
        code: 200,
        msg: String::new(),
        data: format!("{}", String::from_utf8_lossy(&ckm_arr)),
    })
    // 得到邮件模板
    // let collection = db.collection("template");
    // let filter = doc! {"title", "smtp"};
    // let find_options = mongodb::options::FindOneOptions::builder().build();
    // let cursor = collection.find(filter, find_options).expect("查询失败");

    // utils::smtp::data::new()
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