use actix_web::{web, HttpResponse};
use serde::{Serialize,Deserialize};
use crate::error::ServiceError;

#[derive(Serialize, Deserialize)]
pub struct RegisterFrom {
    pub name: String,
    pub email: String,
    pub password: String,
}

pub async fn index(_form: web::Form<RegisterFrom>) -> Result<HttpResponse,ServiceError> {
    Ok(
        HttpResponse::Ok().json({})
    )
    //Err(ServiceError::new(StatusCode::NOT_FOUND, "NotFound".to_string()))
}
