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


lazy_static! {
    static ref conf: Conf = {
       let f = File::open("config.json").expect("读取配置失败").read_to_string();
       serde_json::from_str(f).expect("配置解析失败")
    };
    // static ref REDIS_CONNECTION = {
    //     // lianjie redis
    // };
}

fn init() {
    let conf = Conf {
        redis_connection: String::from(""),
        mongodb_hostname: String::from(""),
        mongodb_port: 0,
        app_hostname: String::from(""),
        app_port: String::from(""),
        session_name: String::from(""),
    };
    let conf: Conf = {
        let mut f = File::open("config.json").expect("读取配置失败");
        let confStr;
        f.read_to_string(&confStr);
        serde_json::from_str(&confStr).expect("配置解析失败")
    };

}

// pub fn init() -> redis::RedisResult<isize>{
//     // connect to redis
//     let client = redis::Client::open("redis://127.0.0.1/6379")?;
//     let mut con = client.get_connection()?;
//     // throw away the result, just make sure it does not fail
//     let _ : () = con.set("my_key", 42)?;
//     // read back the key and return it.  Because the return value
//     // from the function is a result for integer this will automatically
//     // convert into one.
//     con.get("my_key")
// }
