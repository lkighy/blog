use mongodb::Database;

use bson::{Bson, doc, Document};

#[derive(Debug)]
struct smtp_info {
    // id: String,
    email: String, // 唯一
    stmp_addr: String, // smtp
    stmp_port: String, // smtp
    name: String, // 发送人名称
    username: String, // smtp 账号
    password: String, // smtp 密码
}

// 编写宏，要返回的值

// 最后要返回的值 -> Result<smtp_info>
fn smtp_find(db: Database, emal: String) {
    // 得到
    let coll = db.collection("stmp_info");
    let filter = doc! {"email": email};
    let find_options = FindOptions::builder().build();
    let cursor = coll.find(filter, None)?;

    for result in cursor {
        match result {
            Ok(document) => {
                if let Some(email) = docment.get("email").and_then(Bson::as_str)
            },
            Err(e) => println!("{:?}", e),
        }
    }
}

