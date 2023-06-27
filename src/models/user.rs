use crate::schema::users;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::schema::users)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct User {
    pub id: i32,
    pub email: String,
    pub hash: String,
    pub salt: String,
    pub created_at: i32,
    pub updated_at: Option<i32>,
}

#[derive(Insertable)]
#[diesel(table_name = users)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct UserInsert {
    pub email: String,
    pub hash: String,
    pub salt: String,
}

#[derive(Serialize, Deserialize, Selectable, Queryable)]
#[diesel(table_name = users)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct UserResponse {
    pub email: String,
}
