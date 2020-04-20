extern crate native_tls;
use native_tls::{TlsConnector, TlsStream};
use base64::encode;
use std::net::TcpStream;
use std::io;
use std::time::Duration;
use std::io::{Read, Write};

pub struct Client {
    pub conn: TlsStream<TcpStream>, // 连接
    pub tls: bool, // 是否开启 tls 没有用到
    pub auth: [String; 2], // 账号和密码
    pub local_name: String, // 。。。
    pub server_logs: Vec<String>, // 服务器端回应日志
    pub client_logs: Vec<String>, // 客户端日志
}

impl Client {

    fn cmd(&mut self, expect_code: &str, format: String) -> Result<(), io::Error> {
        let mut buffer = [0; 512];
        self.conn.write(&format.as_bytes())?;
        self.conn.read(&mut buffer)?;
        let msg =  String::from_utf8_lossy(&buffer);
        self.client_logs.push(format);
        self.server_logs.push(msg.to_string());
        if !msg.starts_with(expect_code) {
            return Err(io::Error::new(io::ErrorKind::Other, msg));
        }
        Ok(())
    }

    fn ehlo(&mut self) -> std::io::Result<()> {
        self.cmd("250", format!("ehlo {}\r\n", self.local_name))?;
        Ok(())
    }

    fn auth(&mut self) -> std::io::Result<()> {
        // 登录
        self.cmd("334", format!("AUTH LOGIN\r\n"))?;
        self.cmd("334", format!("{}\r\n", encode(&self.auth[0])))?;
        self.cmd("235", format!("{}\r\n", encode(&self.auth[1])))?;
        Ok(())
    }
    fn mail(&mut self, from: String) -> std::io::Result<()> {
        self.cmd("250", format!("MAIL from:<{}>\r\n", from))?;
        Ok(())
    }
    fn rcpt(&mut self, to: Vec<String>) -> std::io::Result<()> {
        for v in to {
            self.cmd("250", format!("RCPT to:<{}>\r\n", v))?;
        }
        Ok(())
    }
    fn data(&mut self, data: String) -> std::io::Result<()> {
        self.cmd("354", format!("data\r\n"))?;
        self.cmd("250", format!("{}\r\n.\r\n", data))?;
        Ok(())
    }
    pub fn send_mail(&mut self, from: String, to: Vec<String>, data: String) -> std::io::Result<()> {
        self.ehlo()?;
        self.auth()?;
        self.mail(from)?;
        self.rcpt(to)?;
        self.data(data)?;
        // self.quit()?;
        Ok(())
    }
    // fn quit(&mut self) -> std::io::Result<()> {
    //     self.cmd("", format!("\r\n.\r\n"))?;
    //     Ok(())
    // }
    pub fn close(&mut self) -> std::io::Result<()> {
        self.conn.shutdown()?;
        Ok(())
    }
}

/// 初始化
pub fn new(
    addr: &str,
    port: u32,
    auth: [String; 2],
    local_name: String,
    timeout: Duration,
    ) -> Result<Client, io::Error>
{
    let mut stream = match TcpStream::connect(format!("{}:{}", addr, port)) {
        Ok(stream) => stream,
        Err(e) => return Err(io::Error::new(io::ErrorKind::Other, format!("创建 TcpStream 失败：{}", e))),
    };
    // 设置读取超时时间
    match stream.set_read_timeout(Some(timeout)) {
        Err(e) => return Err(io::Error::new(io::ErrorKind::Other,format!("设置超时时间失败：{}", e))),
        _ => {},
    }
    let connector = match TlsConnector::new() {
        Ok(con) => con,
        Err(e) => return Err(io::Error::new(io::ErrorKind::Other, format!("创建 TlsConnector 失败：{}", e))),
    };
    let mut stream = match connector.connect(addr, stream) {
        Ok(s) => s,
        Err(e) => return Err(io::Error::new(io::ErrorKind::Other, format!("创建 tls 连接失败：{}", e))),
    };
    let mut logs:Vec<String> = vec![];
    let mut buffer = [0; 512];
    match stream.read(&mut buffer) {
        Err(e) => return Err(io::Error::new(io::ErrorKind::Other, format!("套接字读取失败：{}", e))),
        _ => {},
    }
    logs.push(String::from_utf8_lossy(&buffer).to_string());
    Ok(Client {
        conn: stream,
        tls: true,
        // server_name,
        auth,
        local_name,
        server_logs: (&mut logs).to_owned(),
        client_logs: vec![],
    })
}

pub mod data {
    pub struct Data{
        name: Option<String>, // 发件人名称
        from: Option<String>, // 发件人
        to: Option<Vec<String>>, // 收件人
        cc: Option<Vec<String>>, // 抄送
        subject: Option<String>, // 标题
        body: Option<String>, // 内容
        bcc: Option<Vec<String>>, // 密送
    }

    pub fn new() -> Data {
        Data {
            name: None,
            from: None,
            to: None,
            cc: None,
            subject: None,
            body: None,
            bcc: None,
        }
    }

    impl Data {
        /// 发件人名称
        pub fn name(mut self, name: String) -> Self {
            self.name = Some(name);
            self
        }
        /// 发件人
        pub fn from(mut self, from: String) -> Self {
            self.from = Some(from);
            self
        }
        /// 设置收件人
        pub fn to(mut self, to: Vec<String>) -> Self {
            self.to = Some(to);
            self
        }
        /// 抄送
        pub fn cc(mut self, cc: Vec<String>) -> Self {
            self.cc = Some(cc);
            self
        }
        /// 密送
        pub fn bcc(mut self, bcc: Vec<String>) -> Self {
            self.bcc = Some(bcc);
            self
        }
        /// 标题
        pub fn subject(mut self, subject: String) -> Self{
            self.subject = Some(subject);
            self
        }
        /// 内容
        pub fn body(mut self, body: String) -> Self {
            self.body = Some(body);
            self
        }
        /// 输出
        pub fn to_string(self) -> String {
            // form
            let mut data = vec![];
            if let Some(from) = self.from {
                match self.name {
                    Some(name) => {
                        data.push(format!("From:{} <{}>\r\n", name, from))
                    },
                    None => data.push(format!("From:<{}>\r\n", from)),
                }
            }
            if let Some(to) = self.to {
                for v in to {
                    data.push(format!("To:<{}>\r\n", v))
                }
            }
            if let Some(cc) = self.cc {
                for v in cc {
                    data.push(format!("CC:<{}>\r\n", v))
                }
            }
            if let Some(bcc) = self.bcc {
                for v in bcc {
                    data.push(format!("Bcc:<{}>\r\n", v))
                }
            }
            if let Some(subject) = self.subject {
                data.push(format!("subject:{}\r\n", subject))
            }
            if let Some(body) = self.body {
                data.push(format!("\r\n\r\n{}", body))
            }

            return data.join("")

        }
    }

}

