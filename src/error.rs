use actix_web::{error::{BlockingError, ResponseError}, http::StatusCode, HttpResponse};
use derive_more::Display;

#[derive(Debug, Display, PartialEq)]
pub enum ApiError {
    BadRequest(String),
    NotFound(String),
    Unauthorized(String),
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ErrorResponse {
    errors: Vec<String>,
}

impl ResponseError for ApiError {
    fn error_response(&self) -> HttpResponse {
        match self {
            ApiError::BadRequest(error) => HttpResponse::BadRequest().json::<ErrorResponse>(error.into()),
            ApiError::NotFound(message) => HttpResponse::NotFound().json::<ErrorResponse>(message.into()),
            ApiError::Unauthorized(error) => HttpResponse::Unauthorized().json::<ErrorResponse>(message.into()),
        }
    }
}

impl From<&String> for ErrorResponse {
    fn from(error: &String) -> Self {
        ErrorResponse {
            errors: vec![error.into()],
        }
    }
}