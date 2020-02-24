use chrono::{NaiveDateTime, NaiveDate};

#[derive(Debug, Queryable, Serialize)]
pub struct Author {
    pub id: i32,
    pub author: String,
    pub email: String,
    pub birth: NaiveDate,
    pub introduce: Option<String>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}