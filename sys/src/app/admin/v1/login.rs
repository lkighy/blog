use actix_web::{post, get, web, HttpRequest, Responder};

extern crate redis;
use redis::{Connection, RedisResult};
use std::net::SocketAddr;
use self::redis::Commands;

use serde::{Serialize};

use mongodb::Database;
use bson::{doc, Document};

use mongodb::options::FindOptions;

use crate::utils::{tools};
use crate::service::operate;

use crate::macros::ResultData;

use std::time::Duration;
use std::sync::Mutex;



/// 发送验证码到邮箱
#[get("/send-ckm")]
pub async fn send_ckm(
    req: HttpRequest,
    formdata: web::Json<send_ckm>,
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
    // done 1: 通过 email:ip 查询是否以生成验证码
    // let is_cmk: bool = con.get(format!("{}:{}", email, ip)).expect("hello");
    let cmk:RedisResult<String> = con.get(format!("{}:{}", email, ip));
    if cmk.is_ok() {
        // 如果存在 则 验证是否已经经过一分钟
        // locksendcmk:email:ip
        // done 2: 查询是否生成验证码的时间是否已经经过 1 分钟 -> 提示一分钟后再发送
        let lock: RedisResult<String> = con.get(format!("locksendcmk:{}:{}", email, ip));
        if !lock.is_ok() {
            return web::Json(RedisResult {
                code: 505,
                msg: String::from("请一分钟后再发送验证码"),
                data: String::new(),
            })
        }
    }
    // todo 3: 查询是否以生成三次验证码码 -> 加入临时黑名单 (暂不实现)

    // todo 4: 生成随机数
    let ckm = tools::generate_verify();

    let coll = db.collection("stmp_info");
    // 从数据库中得到数据，账号密码，通过哪个代理发送，以及代理的端口,最后就是模板了喔
    use crate::service::models::SmtpInfo;
    let filter = doc! {"email": "1003027913@qq.com"};
    let smtp_info: SmtpInfo = handle_error_json!(operate::find_one::<SmtpInfo>(filter, None));

    // 发送邮箱
    use crate::utils::{smtp};
    use crate::utils::smtp::{Client, data};

    let auth = [smtp_info.username.clone(), smtp_info.password.clone()];
    let addr = smtp_info.addr;

    // let clent = smtp::new(&smtp_info.addr, &smtp_into.port, [smtp_info.username.clone(), smtp_info.passowrd.clone()], smtp.name.clone);
    let mut client: Client = handle_error_json!(Result, smtp::new(
        &smtp_info.addr,
        smtp_info.port,
        auth,
        smtp_info.name,
        Duration::new(10, 0),
    ));

    let body = data::new()
        .name(smtp_info.name)
        .from(smtp_info.email)
        .to(vec![email])
        .subject("标题".to_string())
        .body(format!("{}", ckm));

    client.send_mail(smtp_info.email.clone(), vec![email], body.to_string());
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