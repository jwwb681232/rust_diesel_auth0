use super::models::{NewUser,User};
use super::schema::users::dsl::*;
use super::Pool;
use crate::diesel::QueryDsl;
use crate::diesel::RunQueryDsl;
use actix_web::{web, Error, HttpResponse,Responder};
use diesel::dsl::{delete,insert_into};
use serde::{Deserialize,Serialize};
use std::vec::Vec;

#[derive(Debug,Deserialize,Serialize)]
pub struct InputUser{
    pub first_name: String,
    pub last_name: String,
    pub email: String,
}

/**************************************************************************/
pub async fn get_users(db: web::Data<Pool>) -> Result<HttpResponse, Error> {
    Ok(web::block(move || get_all_users(db))
        .await
        .map(|user| HttpResponse::Ok().json(user))
        .map_err(|_| HttpResponse::InternalServerError())?)
}

fn get_all_users(pool: web::Data<Pool>) -> Result<Vec<User>, diesel::result::Error> {
    let conn = pool.get().unwrap();
    let items = users.load::<User>(&conn)?;
    Ok(items)
}

/**************************************************************************/
pub async fn get_user_by_id(db: web::Data<Pool>,user_id: web::Path<u32>) -> Result<HttpResponse,Error> {
    Ok(
        web::block(move ||db_get_user_by_id(db,user_id.into_inner()))
            .await
            .map(|user|HttpResponse::Ok().json(user))
            .map_err(|_| HttpResponse::InternalServerError())?,
    )
}

fn db_get_user_by_id(pool: web::Data<Pool>,user_id: u32)->Result<User,diesel::result::Error>{
    let conn = pool.get().unwrap();
    users.find(user_id).execute(&conn)?
}


/**************************************************************************/
pub async fn add_user(db: web::Data<Pool>,item: web::Json<InputUser>) -> Result<HttpResponse,Error> {
    Ok(
        web::block(move || add_single_user(db,item))
            .await
            .map(|user|HttpResponse::Ok().json(user))
            .map_err(|_|HttpResponse::InternalServerError())?
    )
}

fn add_single_user(pool: web::Data<Pool>,item: web::Json<InputUser>)->Result<User,diesel::result::Error>{
    let conn = pool.get().unwrap();
    let new_user = NewUser{
        first_name: &item.first_name,
        last_name: &item.last_name,
        email: &item.email,
        created_at: chrono::Local::now().naive_local(),
    };

    let res = insert_into(users).values(&new_user).execute(&conn)?;
    Ok(res)
}


/**************************************************************************/
pub async fn delete_user(db: web::Data<Pool>,user_id: web::Path<u32>) -> Result<HttpResponse,Error> {
    Ok(
        web::block(move || delete_single_user(db,user_id.into_inner()))
            .await
            .map(|user|HttpResponse::Ok().json(user))
            .map_err(|_|HttpResponse::InternalServerError())?
    )
}

fn delete_single_user(pool: web::Data<Pool>,user_id: u32)-> Result<usize,diesel::result::Error> {
    let conn = pool.get().unwrap();
    let count = delete(users.find(user_id)).execute(&conn)?;
    Ok(count)
}