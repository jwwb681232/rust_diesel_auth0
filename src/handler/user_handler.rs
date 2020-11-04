use crate::diesel::RunQueryDsl;
use crate::model::user_model::User;
use crate::rim::error::{ApiError, ApiResult};
use crate::schema::users::dsl::*;

use actix_web::{web, HttpResponse};
use std::vec::Vec;

use crate::Pool;

pub async fn get_users(db: web::Data<Pool>) -> ApiResult<HttpResponse> {
    Ok(web::block(move || get_all_users(db))
        .await
        .map(|user| HttpResponse::Ok().json(user))
        .map_err(|_| ApiError::new(500, "Internal Server Error"))?)
}

fn get_all_users(pool: web::Data<Pool>) -> Result<Vec<User>, diesel::result::Error> {
    let conn = pool.get().unwrap();
    let items = users.load::<User>(&conn)?;
    Ok(items)
}
