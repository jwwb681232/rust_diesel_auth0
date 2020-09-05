use actix_web::{get, web, App, HttpResponse, HttpServer, error::ResponseError, http::StatusCode};

use thiserror::Error;

#[derive(Debug, Error)]
pub enum RDAError {
    #[error("Resource Not Found")]
    NotFound,

    #[error("You are forbidden to access request")]
    Forbidden,

    #[error("Unknown Internal Error")]
    Unknown,
}

impl RDAError {
    pub fn name(&self) -> String {
        match self {
            RDAError::NotFound => "NotFound".to_string(),
            RDAError::Forbidden => "Forbidden".to_string(),
            RDAError::Unknown => "Unknown".to_string(),
        }
    }
}

impl ResponseError for RDAError {
    fn status_code(&self) -> StatusCode {
        match *self {
            RDAError::NotFound => StatusCode::NOT_FOUND,
            RDAError::Forbidden => StatusCode::FORBIDDEN,
            RDAError::Unknown => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }

    fn error_response(&self) -> HttpResponse {
        let status_code = self.status_code();

        let error_response = ErrorResponse {
            code: status_code.as_u16(),
            error: self.to_string(),
            message: self.name(),
        };

        HttpResponse::build(status_code).json(error_response)
    }
}

#[derive(Serialize)]
struct ErrorResponse {
    code: u16,
    error: String,
    message: String,
}
