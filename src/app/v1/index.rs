use actix_web::get;
use crate::conf::DB;
use bson::{Bson, doc};
use mongodb::options::FindOptions;

#[get("/")]
pub async fn index() -> String {
    // String::from("好")
    // HttpResponse::Ok().body("hello")
    let collection = DB.collection("test");
    let filter = doc! {"title": "who"};
    let find_options = FindOptions::builder().sort(doc! {"title": 1}).build();
    let cursor = collection.find(filter, find_options).expect("查询失败");


    for result in cursor {
        match result {
            Ok(document) => {
                if let Some(title) = document.get("title").and_then(Bson::as_str) {
                    return format!("title: {}", title)
                } else {
                    return String::from("no title found")
                }
            },
            Err(_) => return String::from("error"),
        };
    }

    String::from("完结")

}

