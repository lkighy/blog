use mongodb::Collection;
use mongodb::error::Error;

use bson::{Bson, Document};

/// let coll = db.collection("smtpInfo");
/// let document = doc! {"email": "100@qq.com"};
/// let filter = doc! {"email": "100@qq.com"};
/// let data = coll.find_one(document, options);
pub fn find_one<T>(coll: Collection, document: Document, options: Option<Document>) -> Result<T, Error> {
    let data = coll.find_one(document, options)??;
    bson::from_bson::<T>(bson::Bson::Document(data))?
}

pub fn find<T>(coll: Collection, document: Document, options: Option<Document>) -> Result<Vec<Document>, Error> {
    let cursor = coll.find(document, options)?;
    let mut results: Vec<Document> = Vec::new();
    for result in cursor {
        results.append(*result?);
    }
    Ok(results)
}
