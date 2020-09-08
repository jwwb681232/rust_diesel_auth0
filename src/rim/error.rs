use std::{fmt::{Display, Formatter, Result as fmtResult}};
use serde::{Serialize, Deserialize};
use actix_web::{ResponseError, HttpResponse};
use actix_web::http::StatusCode;
use serde_json::json;
use diesel::result::Error;

pub type ApiResult<T> = actix_web::Result<T,ApiError>;

#[derive(Debug, Deserialize, Serialize)]
pub struct ApiError {
    pub status_code: u16,
    pub message: &'static str,
}

impl ApiError {
    pub fn new(status_code: u16, message: &'static str) -> ApiError {
        ApiError { status_code, message }
    }
}

impl Display for ApiError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmtResult {
        write!(f, "{:?}", self)
    }
}

impl ResponseError for ApiError {
    fn error_response(&self) -> HttpResponse {
        HttpResponse::build(StatusCode::default()).json(
            json!({
                "status_code":self.status_code,
                "message": self.message
            })
        )
    }
}

impl From<diesel::result::Error> for ApiError {
    fn from(e: Error) -> Self {
        match e {
            _=> ApiError::new(500,"Database Error")
        }
    }
}

impl From<serde_json::error::Error> for ApiError{
    fn from(_: serde_json::error::Error) -> Self {
        ApiError::new(500,"Database Error")
    }
}
