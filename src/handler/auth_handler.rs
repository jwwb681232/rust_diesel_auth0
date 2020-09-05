use actix_web::{web, HttpResponse};
use serde::{Serialize,Deserialize};
use crate::error::Error;

#[derive(Serialize, Deserialize)]
pub struct RegisterFrom {
    pub name: String,
    pub email: String,
    pub password: String,
}

pub async fn index(_form: web::Form<RegisterFrom>) -> Result<HttpResponse,Error> {
    Err(Error {
        msg: "an example error message".to_string(),
        status: 200,
    })
    /*Ok(
        HttpResponse::Ok().json(RegisterFrom {
            name: form.name.to_string(),
            email: form.email.to_string(),
            password: form.password.to_string(),
        })
    )*/
}
