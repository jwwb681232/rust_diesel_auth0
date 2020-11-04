use crate::rim::error::ApiError;
use actix_web::error::Error;
use actix_web::error::{InternalError, UrlencodedError};
use actix_web::{HttpRequest, HttpResponse};

pub fn form_error_handler(err: UrlencodedError, _req: &HttpRequest) -> Error {
    let response = match &err {
        UrlencodedError::Chunked => HttpResponse::Ok().json(ApiError {
            status_code: 200,
            message: Box::leak(err.to_string().into_boxed_str()),
        }),
        UrlencodedError::Overflow { .. } => HttpResponse::Ok().json(ApiError {
            status_code: 200,
            message: Box::leak(err.to_string().into_boxed_str()),
        }),
        UrlencodedError::UnknownLength => HttpResponse::Ok().json(ApiError {
            status_code: 200,
            message: Box::leak(err.to_string().into_boxed_str()),
        }),
        UrlencodedError::ContentType => HttpResponse::Ok().json(ApiError {
            status_code: 200,
            message: Box::leak(err.to_string().into_boxed_str()),
        }),
        UrlencodedError::Parse => HttpResponse::Ok().json(ApiError {
            status_code: 200,
            message: Box::leak(err.to_string().into_boxed_str()),
        }),
        UrlencodedError::Payload(e) => HttpResponse::Ok().json(ApiError {
            status_code: 200,
            message: Box::leak(e.to_string().into_boxed_str()),
        }),
    };

    InternalError::from_response(err, response).into()
}
