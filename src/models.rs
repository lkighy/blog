use chrono::{NaiveDateTime, NaiveDate};
use serde_json;

#[derive(Debug, Queryable, Serialize)]
pub struct Author {
    pub id: i32,
    pub author: String,
    pub email: String,
    pub hobby: String,
    pub birth: NaiveDate,
    pub introduce: Option<String>,
    pub lines: serde_json::Value,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

