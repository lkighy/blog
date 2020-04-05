extern crate redis;
// use redis::Commands;
use std::fs::File;
use serde::{Deserialize};
use serde_json;
use std::io::Read;

#[derive(Deserialize, Debug)]
pub struct Conf {
    redis_connection: String, // redis://127.0.0.1:6379
    mongodb_hostname: String, // 127.0.0.1
    mongodb_port: i32, // mongodb 27017
    app_hostname: String, // app 主键 127.0.0.1
    app_port: String, // app 绑定端口
    session_name: String, // session 键名称
}

// #[derive(Deserialize, Debug)]
// pub struct Conf {
//     redis_connection: &'static str, // redis://127.0.0.1:6379
//     mongodb_hostname: &'static str, // 127.0.0.1
//     mongodb_port: i32, // mongodb 27017
//     app_hostname: &'static str, // app 主键 127.0.0.1
//     app_port: &'static str, // app 绑定端口
//     session_name: &'static str, // session 键名称
// }

pub const conf:Conf = {
    let mut f = File::open("config.json").expect("读取配置文件失败");
    let mut confStr = String::new();
    f.read_to_string(&mut confStr).expect("转换为字符串失败");
    serde_json::from_str(&confStr).expect("配置解析失败")
};

