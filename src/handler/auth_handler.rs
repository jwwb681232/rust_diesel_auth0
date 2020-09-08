use crate::model::auth_model::{NewUser, User};
use crate::schema::users::dsl::*;
use crate::diesel::QueryDsl;
use crate::diesel::RunQueryDsl;
use crate::rim::error::{ApiResult, ApiError};

use actix_web::{web, HttpResponse};
use diesel::dsl::{delete, insert_into};
use serde::{Deserialize, Serialize};
use std::vec::Vec;

use crate::Pool;

#[derive(Debug, Deserialize, Serialize)]
pub struct InputUser {
    pub first_name: String,
    pub last_name: String,
    pub email: String,
}

/**************************************************************************/
pub async fn get_users(db: web::Data<Pool>) -> ApiResult<HttpResponse> {
    Ok(web::block(move || get_all_users(db))
        .await
        .map(|user| HttpResponse::Ok().json(user))
        .map_err(|_| ApiError::new(500, "Internal Server Error"))?
    )
}

fn get_all_users(pool: web::Data<Pool>) -> Result<Vec<User>, diesel::result::Error> {
    let conn = pool.get().unwrap();
    let items = users.load::<User>(&conn)?;
    Ok(items)
}

/**************************************************************************/
pub async fn get_user_by_id(db: web::Data<Pool>, user_id: web::Path<u32>) -> ApiResult<HttpResponse> {
    Ok(
        web::block(move || db_get_user_by_id(db, user_id.into_inner()))
            .await
            .map(|user| HttpResponse::Ok().json(user))
            .map_err(|_| ApiError::new(500, "Internal Server Error"))?,
    )
}

fn db_get_user_by_id(pool: web::Data<Pool>, user_id: u32) -> Result<User, diesel::result::Error> {
    let conn = pool.get().unwrap();
    users.find(user_id).first(&conn)
}


/**************************************************************************/
pub async fn add_user(db: web::Data<Pool>, item: web::Json<InputUser>) -> ApiResult<HttpResponse> {
    Ok(
        web::block(move || add_single_user(db, item))
            .await
            .map(|inserted| HttpResponse::Ok().json(inserted))
            .map_err(|_| ApiError::new(500, "Internal Server Error"))?
    )
}

fn add_single_user(pool: web::Data<Pool>, item: web::Json<InputUser>) -> Result<usize, diesel::result::Error> {
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


/**************************************************************************/
pub async fn delete_user(db: web::Data<Pool>, user_id: web::Path<u32>) -> ApiResult<HttpResponse> {
    Ok(
        web::block(move || delete_single_user(db, user_id.into_inner()))
            .await
            .map(|user| HttpResponse::Ok().json(user))
            .map_err(|_| ApiError::new(500, "Internal Server Error"))?
    )
}

fn delete_single_user(pool: web::Data<Pool>, user_id: u32) -> Result<usize, diesel::result::Error> {
    let conn = pool.get().unwrap();
    let count = delete(users.find(user_id)).execute(&conn)?;
    Ok(count)
}


/**************************************************************************/
