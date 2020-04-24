// use diesel::prelude::*;
use chrono::NaiveDateTime;

//
use crate::schema::posts;

#[derive(Queryable, Serialize, Deserialize)]
pub struct Post {
    pub id: i64,
    pub created_at: NaiveDateTime, // Local::now().naive_local()
    pub title: String,
    pub body: String,
    pub published: bool,
    //
    pub author_id: i64,
}

#[derive(Insertable)]
#[table_name = "posts"]
pub struct NewPost<'a> {
    pub title: &'a str,
    pub body: &'a str,
    pub author_id: i64,
}
