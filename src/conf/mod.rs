// pub mod conf;
extern crate redis;
// use redis::Commands;
use std::fs::File;
use serde::{Deserialize};
use serde_json;
use std::io::Read;
use lazy_static::lazy_static;

use mongodb::{Client, Database};
use mongodb::options::{ClientOptions, StreamAddress};
// Struct mongodb::options::StreamAddress

#[derive(Deserialize, Debug)]
pub struct Conf {
    pub redis_connection: &'static str, // redis://127.0.0.1:6379
    pub mongodb_hostname: &'static str, // 127.0.0.1
    pub mongodb_port: u16, // mongodb 27017
    pub mongodb_name: &'static str, // mongodb 数据库名称
    pub app_hostname: &'static str, // app 主键 127.0.0.1
    pub app_port: &'static str, // app 绑定端口
    pub session_name: &'static str, // session 键名称
}

lazy_static! {
    pub static ref CONF:Conf = {
        let mut f = File::open("config.json").expect("读取配置文件失败");
        let mut conf_str = String::new();
        f.read_to_string(&mut conf_str).expect("转换为字符串失败");
        let conf_str: &'static str = Box::leak(conf_str.into_boxed_str());
        serde_json::from_str(&conf_str).expect("配置解析失败")
    };
    // 初始化数据库
    // redis
    // mongodb
    pub static ref DB:Database = {
        println!("初始化mongodb");
        let options = ClientOptions::builder()
        .hosts(vec![
            StreamAddress {
                hostname: CONF.mongodb_hostname.into(),
                port: Some(CONF.mongodb_port),
            }
        ])
        .build();
        let client = Client::with_options(options).expect("连接mongodb 失败");
        // let client = Client::with_uri_str(format!("mongodb://{}:{}", "127.0.0.1", 27017)).expect("连接mongodb 失败");
        client.database(CONF.mongodb_name)
    };
}

// fn init() {
//     let a = Conf {
//         redis_connection: "a",
//         mongodb_hostname: "a",
//         mongodb_port: i32,
//         mongodb_name: "a",
//         app_hostname: "a",
//         app_port: "a",
//         session_name: "a",
//     };
//
//     // format!("{}", a.mongodb_hostname.to_string());
// }
