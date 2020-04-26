// 登录
// 1.
// 发送验证码
use actix_web::{post, get, web, HttpRequest};
use actix_redis::{RedisActor};
use actix::Addr;

#[get("/send-ckm")]
pub async fn send_ckm(req: HttpRequest, _redis: web::Data<Addr<RedisActor>>) -> String {
    // 获取 ip 地址，
    // 查询该 ip 是否已被拉黑
    // 查询该 ip 是否已经提交了三次错误验证码
    // 查询 redis 缓存中是否有 验证码
    // 有
    // 警告: 以提交无法重复生成。并且在 redis 中添加一次警告 三次警告则拉黑该 ip 地址
    // 无
    // 生成随机数
    // 将 ip 与 随机码绑定，添加至 redis 库中。
    // 发送邮件到邮箱中。等待输入
    // redis.send()

    format!("{:?}", req.connection_info())
}

#[post("/verify-ckm")]
pub async fn verify_ckm(_redis: web::Data<Addr<RedisActor>>) -> String {
    // 获取 ip
    // 查询该 ip 下的验证码是否相同
    String::from("verify-ckm")
}

// redis 关于验证登录这块的参数格式
// 1. 记录 ip
// 2. 记录发送次数，错误次数
// 3. 验证码
// 4. 