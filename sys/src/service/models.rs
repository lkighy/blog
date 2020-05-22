use mongodb::{Database, Cursor};

use mongodb::Collection;
use mongodb::error::Error;
use mongodb::options::FindOptions;

use bson::{Bson, doc, Document};
use bson::ordered::OrderedDocument;

use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
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

// pub fn find_one<T>(coll: Collection, document: Document, options: Option<Document>) -> Result<T, Error> {
//     let data = coll.find_one(document, options)??;
//     bson::from_bson::<T>(bson::Bson::Document(data))?
// }
// pub fn find<T>(coll: Collection, document: Document, options: Option<Document>) -> Result<Vec<Document>, Error> {
//     let cursor = coll.find(document, options)?;
//     let mut results: Vec<Document> = Vec::new();
//     for result in cursor {
//         results.append(*result?);
//     }
//     Ok(results)
// }

