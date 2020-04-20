// use mail_demo::Client;
use mail_demo::data;
// use mail_demo::Data::new;
use mail_demo::new;
use std::time::Duration;

fn main() {
    let mut client = match new(
        "smtp.163.com",
        587,
        ["123@qq.com".to_string(), "123456".to_string()],
        "smtp.163.com".to_string(),
        Duration::new(10, 0),
    ) {
        Ok(client) => client,
        Err(e) => panic!(e),
    };
    let data = data::new()
        .name("青丘".to_string())
        .from(String::from("123@qq.com"))
        .to(vec!["1003027913@qq.com".to_string()])
        .subject("发送验证码".to_string())
        .body("验证码<br> abc".to_string());

    match client.send_mail(
        String::from("123@qq.com"),
        vec!["3456@qq.com".to_string()],
        data,
    ) {
        _ => {},
        Err(e) => panic!(e),
    }
    client.close();
    println!("server_logs\r\n{}\r\n", client.server_logs.join("\r\n"));
    println!("client_logs\r\n{}\r\n", client.client_logs.join("\r\n"));

}

