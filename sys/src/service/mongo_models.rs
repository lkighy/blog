use mongodb::Database;

use mongodb::error::Error;

use bson::{Bson, doc, Document};

#[derive(Debug)]
struct smtp_info {
    id: String,
    email: String, // 唯一
    addr: String, // smtp
    port: String, // smtp
    name: String, // 发送人名称
    username: String, // smtp 账号
    password: String, // smtp 密码
}

// 编写宏，要返回的值
impl smtp_info {
    fn find(db: Database, email: String) -> Result<(), Error> {
        let coll = db.collection("stmp_info");
        let filter = doc! {"email": email};
        let cursor = coll.find(filter, None)?;

    }
}

// 最后要返回的值 -> Result<smtp_info>
fn smtp_find(db: Database, emal: String) {
    // 得到
    let coll = db.collection("stmp_info");
    let filter = doc! {"email": email};
    let find_options = FindOptions::builder().build();
    let cursor = coll.find(filter, None)?;

    let results: Vec<Result<Document>> = cursor.collect();

    let result = results[0];

    // for result in cursor {
    //     match result {
    //         Ok(document) => {
    //             if let Some(email) = docment.get("email").and_then(Bson::as_str)
    //         },
    //         Err(e) => println!("{:?}", e),
    //     }
    // }
}

