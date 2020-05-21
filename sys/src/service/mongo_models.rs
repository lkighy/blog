use mongodb::Database;

use mongodb::error::Error;
use mongodb::options::FindOptions;

use bson::{Bson, doc, Document};

#[derive(Debug)]
struct SmtpInfo {
    #[serde(rename = "_id")]
    pub id: bson::oid::ObjectId,
    pub email: String, // 唯一
    pub addr: String, // smtp
    pub port: String, // smtp
    pub name: String, // 发送人名称
    pub username: String, // smtp 账号
    pub password: String, // smtp 密码
}

// 编写宏，要返回的值
impl SmtpInfo {
    fn find_one(&mut self, db: Database, document: Document, options: Option<Document>) -> Result<(), Error> {
        let coll = db.collection("stmp_info");
        let data = coll.find_one(document, options)??;
        self = *bson::from_bson::<SmtpInfo>(bson::Bson::Document(data))?;
    }
}

// // 最后要返回的值 -> Result<smtp_info>
// fn smtp_find(db: Database, emal: String) {
//     // 得到
//     let coll = db.collection("stmp_info");
//     let filter = doc! {"email": email};
//     let find_options = FindOptions::builder().build();
//     let cursor = coll.find(filter, None)?;
//
//     let results: Vec<Result<Document>> = cursor.collect();
//
//     let result = results[0];
//
// }

