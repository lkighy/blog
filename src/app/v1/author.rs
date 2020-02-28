use rocket_contrib::json::JsonValue;
use diesel::prelude::*;

extern crate diesel;

use crate::PgDbConn;
use crate::models::*;
use crate::app;
use crate::schema::authors::dsl::*;

#[post("/")]
pub fn info(conn: PgDbConn) -> JsonValue {

    let results = authors
//        .load::<Author>(&*conn);
        .get_result::<Author>(&*conn);

    let r = match results {
//        Ok(res) => json!(res),
        Ok(res) => {
            json!(app::FormatJson {
                code: 200,
                msg: String::new(),
                data: res,
            })
        },
//        Ok(res) => panic!("test 出错"),
        Err(e) => {
            json!(app::FormatJson {
                code: 500,
                msg: String::from(format!("{:?}", e)),
                data: String::new()
            })
        },
    };
    r
}

