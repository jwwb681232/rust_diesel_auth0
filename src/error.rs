/*
use actix_web::{error,Result};

#[derive(Debug)]
pub struct RDAError{
    name:&'static str,
}

*/
use std::fmt::{Display, Formatter, Result as FmtResult};
use actix_web::{web, ResponseError};
use serde_json::{json, to_string_pretty};
use serde::Serialize;
use actix_web::http::StatusCode;

#[derive(Debug, Serialize)]
pub struct Error {
    pub msg: String,
    pub status: u16,
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "{}", to_string_pretty(self).unwrap())
    }
}

impl ResponseError for Error {
    fn error_response(&self) -> web::HttpResponse {
        let err_json = json!({ "error": self.msg });
        web::HttpResponse::build(StatusCode::from_u16(self.status).unwrap()).json(err_json)
    }
}
