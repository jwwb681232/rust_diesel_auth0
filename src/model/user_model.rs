use crate::schema::*;
use serde::{Serialize,Deserialize};

#[derive(Debug,Serialize,Deserialize,Queryable)]
pub struct User{
    pub id: u32,
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub password: String,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
}

#[derive(Insertable,Debug)]
#[table_name="users"]
pub struct NewUser<'a> {
    pub first_name: &'a str,
    pub last_name: &'a str,
    pub email: &'a str,
    pub password: &'a str,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct InputUser {
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub password: String,
}
