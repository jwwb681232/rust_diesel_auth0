// use std::fmt::{Display, Formatter, Result as FmtResult};
// use actix_web::{web, ResponseError};
// use serde_json::{json, to_string_pretty};
// use serde::Serialize;
// use actix_web::http::StatusCode;

// #[derive(Debug, Serialize)]
// pub struct Error {
//     pub msg: String,
//     pub status: u16,
// }

// impl Display for Error {
//     fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
//         write!(f, "{}", to_string_pretty(self).unwrap())
//     }
// }

// impl ResponseError for Error {
//     fn error_response(&self) -> web::HttpResponse {
//         let err_json = json!({ "error": self.msg });
//         web::HttpResponse::build(StatusCode::from_u16(self.status).unwrap()).json(err_json)
//     }
// }

use crate::models::response::ResponseBody;
use actix_web::{http::StatusCode};

pub struct ServiceError{
    pub http_status:StatusCode,
    pub body: ResponseBody<String>,
}


impl ServiceError {
    pub fn new(http_status:StatusCode,message:String)->ServiceError{
        ServiceError{
            http_status,
            body:ResponseBody{
                message,
                data: String::new()
            }
        }
    }
}