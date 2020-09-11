use serde::{Serialize, Deserialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Response<T> {
    pub status_code: u16,
    pub message: &'static str,
    pub data: Option<T>,
}
