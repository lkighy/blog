// 登录
// 1.
// 发送验证码
use actix_web::{post, get, web, HttpRequest};
// use actix_redis::{RedisActor};
// use actix::Addr;

// use redis;
// use redis::Commands;
extern crate redis;
use redis::Connection;
use std::net::SocketAddr;
use self::redis::Commands;
use std::sync::Mutex;

#[get("/send-ckm")]
pub async fn send_ckm(req: HttpRequest, redis_con: web::Data<Mutex<Connection>>) -> String {
    // 获取 ip 地址，
    let addr = match req.peer_addr() {
        Some(addr) => addr,
        None => panic!(),
    };
    let ip = match addr {
        SocketAddr::V4(ipv4) => format!("{:?}", ipv4.ip()),
        SocketAddr::V6(ipv6) => format!("{:?}", ipv6.ip()),
    };
    // todo 1: 查询该 ip 是否已被拉黑

    // let _: () = *redis_a.sadd("blacklist", &ip).unwrap();
    // let _:() = redis_con.lock().unwrap().sadd("blacklist", &ip).unwrap();
    let mut redis_a = redis_con.lock().unwrap();
    let _:() = redis_a.sadd("blacklist", &ip).unwrap();
    // let _:() = redis_con.sadd("blacklist", &ip).unwrap();

    // let _:() = *redis_con.get_ref().sadd("blacklist", &ip).unwrap();

    // redis.get_ref().

    // 查询该 ip 是否已经提交了三次错误验证码
    // 查询 redis 缓存中是否有 验证码
    // 有
    // 警告: 以提交无法重复生成。并且在 redis 中添加一次警告 三次警告则拉黑该 ip 地址
    // 无
    // 生成随机数
    // 将 ip 与 随机码绑定，添加至 redis 库中。
    // 发送邮件到邮箱中。等待输入
    // redis.send()

    format!("{}", ip)
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
// 4. 