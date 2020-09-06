use std::{fmt::{Display,Formatter,Result as fmtResult}};
use serde::{Serialize,Deserialize};
use actix_web::{ResponseError, HttpResponse};
use actix_web::http::StatusCode;
use serde_json::json;

#[derive(Debug,Deserialize,Serialize)]
pub struct ApiError{
    pub status_code:u16,
    pub message: String,
}

impl ApiError{
    pub fn new(status_code:u16,message:String) ->ApiError {
        ApiError{ status_code, message }
    }
}

impl Display for ApiError{
    fn fmt(&self, f: &mut Formatter<'_>) -> fmtResult {
        write!(f,"{}",self.message.as_str())
    }
}

impl ResponseError for ApiError{
    fn error_response(&self)->HttpResponse{
        let status_code = match StatusCode::from_u16(self.status_code) {
            Ok(code) => code,
            Err(_) => StatusCode::INTERNAL_SERVER_ERROR,
        };

        let message = match status_code.as_u16() < 500 {
            true => self.message.clone(),
            false=> "Internal Server Error".to_string()
        };

        HttpResponse::build(status_code).json(json!({ "message": message }))
    }
}

pub async fn index() -> actix_web::Result<String, ApiError> {
    Err(ApiError::new(500,"NotFound".to_string()))
}
