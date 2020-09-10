use actix_web::error::{JsonPayloadError, InternalError};
use actix_web::{HttpRequest, HttpResponse};
use actix_web::error::Error;
use crate::rim::error::ApiError;

pub fn json_error_handler(err: JsonPayloadError, _req: &HttpRequest) -> Error {
    let response = match &err {
        JsonPayloadError::Deserialize(e) =>
            HttpResponse::Ok().json(ApiError { status_code: 200, message: Box::leak(e.to_string().into_boxed_str()) }),
        JsonPayloadError::Payload(e) =>
            HttpResponse::Ok().json(ApiError { status_code: 200, message: Box::leak(e.to_string().into_boxed_str()) }),
        JsonPayloadError::ContentType | JsonPayloadError::Overflow =>
            HttpResponse::Ok().json(ApiError { status_code: 200, message: Box::leak(err.to_string().into_boxed_str()) }),
    };

    //ApiError::new()
    InternalError::from_response(err, response).into()
}
