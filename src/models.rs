use diesel::{r2d2::ConnectionManager, PgConnection};
use super::schema::*;
use serde::{Deserialize, Serialize};

pub type Pool = r2d2::Pool<ConnectionManager<PgConnection>>;

#[derive(Identifiable, Debug, Deserialize, Serialize, Queryable, Insertable)]
#[table_name = "users"]
#[primary_key(user_id)]
pub struct User {
    pub user_id: i32,
    pub name: String,
    pub password: String,
    pub email: String,
    pub created_at: chrono::NaiveDateTime,
}

#[derive(Identifiable, Associations, Debug, Deserialize, Serialize, Queryable, Insertable)]
#[belongs_to(User, foreign_key = "user_id")]
#[primary_key(bucket_id)]
#[table_name = "buckets"]
pub struct Bucket {
    pub bucket_id: i32,
    pub name: String,
    pub date_created: chrono::NaiveDateTime,
    pub date_modified: chrono::NaiveDateTime,
    pub user_id: i32
}

#[derive(Identifiable, Associations, Debug, Deserialize, Serialize, Queryable, Insertable)]
#[belongs_to(Bucket, foreign_key = "bucket_id")]
#[primary_key(item_id)]
#[table_name = "items"]
pub struct Item {
    pub item_id: i32,
    pub name: String,
    pub bucket_id: i32,
    pub date_created: chrono::NaiveDateTime,
    pub date_modified: chrono::NaiveDateTime,
    pub completed: bool
}