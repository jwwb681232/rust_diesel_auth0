use crate::rim::error::{ApiResult,ApiError};

pub async fn index() -> ApiResult<String> {
    Err(ApiError::new(500, "NotFound"))
}
