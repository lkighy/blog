// use std::fs::{self, File};
// use std::io::Read;
// use std::io;
// use serde_json;
// use serde::{Deserialize};


// #[derive(Deserialize, Debug)]
// pub struct Conf {
//     redis_connection: String,
//     mongodb_hostname: String,
//     mongodb_port: i32,
//     app_hostname: String,
//     app_port: String,
//     session_name: String,
// }

// #[derive(Deserialize, Debug)]
// pub struct Conf {
//     redis_connection: &'static str, // redis://127.0.0.1:6379
//     mongodb_hostname: &'static str, // 127.0.0.1
//     mongodb_port: i32, // mongodb 27017
//     app_hostname: &'static str, // app 主键 127.0.0.1
//     app_port: &'static str, // app 绑定端口
//     session_name: &'static str, // session 键名称
// }


mod conf;

fn main() {

    // let mut f = File::open("config.json").expect("读取配置文件失败");
    // let mut confStr = String::new();
    // f.read_to_string(&mut confStr).expect("转换为字符串失败");
    // let conf:Conf = serde_json::from_str(&confStr).expect("配置解析失败");
    //
    // println!("{:?}", conf);
    println!("{:?}", conf::conf::conf)
}
