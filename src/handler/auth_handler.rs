use crate::model::user_model::{NewUser, User, InputUser, RegisterUser};
use crate::schema::users::dsl::*;
use crate::diesel::QueryDsl;
use crate::diesel::RunQueryDsl;
use crate::rim::error::{ApiResult, ApiError};

use actix_web::{web, HttpResponse};
use diesel::dsl::{delete, insert_into};
use std::vec::Vec;

use crate::Pool;

pub async fn register(db: web::Data<Pool>, item: web::Json<RegisterUser>) -> ApiResult<HttpResponse> {
    Ok(
        web::block(move || add_user(db, item))
            .await
            .map(|inserted| HttpResponse::Ok().json(inserted))
            .map_err(|_| ApiError::new(500, "Internal Server Error"))?
    )
}

fn add_user(pool: web::Data<Pool>, item: web::Json<RegisterUser>) -> Result<usize, diesel::result::Error> {
    let conn = pool.get().unwrap();
    let new_user = NewUser {
        first_name: &item.first_name,
        last_name: &item.last_name,
        email: &item.email,
        created_at: chrono::Local::now().naive_local(),
    };

    Ok(
        insert_into(users).values(&new_user).execute(&conn)?
    )
}
