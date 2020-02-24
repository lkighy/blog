#![feature(proc_macro_hygiene, decl_macro)]

use rocket_contrib::json::{Json, JsonValue};

extern crate diesel;

use crate::PgDbConn;
use crate::models::*;

use diesel::prelude::*;


#[get("/")]
pub fn info(conn: PgDbConn) -> String {
    use crate::schema::authors::dsl::*;

    let mut s = String::from("");

    let results = authors
//        .limit(1)
        .load::<Author>(&*conn);

    match results {
        Ok(res) => println!("{:?}", res),
        Err(e) => { s.push_str(&e.to_string())},
    }

    s
//    let res = match resultsOption {
//        Ok(results) => results,
//        Err(e) => panic!("error"),
//    };

//    Json!(results)
}

