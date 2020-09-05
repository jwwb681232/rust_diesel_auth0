use actix_web::{error::ResponseError,HttpResponse};
use derive_more::Display;

#[derive(Debug,Display)]
pub enum ServiceError{
    #[display(fmt="Internal Server Error")]
    #[allow(dead_code)]
    InternalServerError,

    #[display(fmt="Bad Request: {}", _0 )]
    #[allow(dead_code)]
    BadRequest(String),

    #[display(fmt="JWKSFetchError")]

    #[allow(dead_code)]
    JWKSFetchError,
}

impl ResponseError for ServiceError {
    fn error_response(&self) ->HttpResponse {
        match self{
            ServiceError::InternalServerError=>{
                HttpResponse::InternalServerError().json("Internal Server Error, Please try later")
            },
            ServiceError::BadRequest(ref message) => {
                HttpResponse::BadRequest().json(message)
            },
            ServiceError::JWKSFetchError=>{
                HttpResponse::InternalServerError().json("Could not fetch JWKS")
            },
        }
    }
}
