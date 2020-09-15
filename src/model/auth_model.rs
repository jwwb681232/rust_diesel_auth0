use crate::schema::*;
use serde::{Serialize, Deserialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct RegisterForm {
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub password: String,
    pub confirm_password: String,
}

#[derive(Insertable,Debug)]
#[table_name="users"]
pub struct InsertRegisterData{
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub password: String,
    pub updated_at: chrono::NaiveDateTime,
}
