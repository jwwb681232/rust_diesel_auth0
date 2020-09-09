use actix_web::error::{JsonPayloadError, InternalError};
use actix_web::{HttpRequest, HttpResponse};
use actix_web::error::Error;
use serde_json::json;
use crate::rim::error::ApiError;

pub fn json_error_handler(err: JsonPayloadError, _req: &HttpRequest) -> Error {
    let response = match &err {
        JsonPayloadError::Deserialize(e) => {
            HttpResponse::Ok().json(ApiError{ status_code: 200, message: Box::leak(e.to_string().into_boxed_str()) })
        }
        _ => {
            HttpResponse::UnsupportedMediaType().content_type("plan/text").body(err.to_string())
        }
    };

    //ApiError::new()
    InternalError::from_response(err, response).into()
}
