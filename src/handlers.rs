use super::models::{NewUser,User};
use super::schema::users::dsl::*;
use super::Pool;
use crate::diesel::QueryDsl;
use crate::diesel::RunQueryDsl;
use actix_web::{web, Error, HttpResponse, HttpRequest};
use diesel::dsl::{delete,insert_into};
use serde::{Deserialize,Serialize};
use std::vec::Vec;
use crate::errors::RDAError;

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
    users.find(user_id).first(&conn)
}


/**************************************************************************/
pub async fn add_user(db: web::Data<Pool>,item: web::Json<InputUser>) -> Result<HttpResponse,Error> {
    Ok(
        web::block(move || add_single_user(db,item))
            .await
            .map(|inserted|HttpResponse::Ok().json(inserted))
            .map_err(|_|HttpResponse::InternalServerError())?
    )
}

fn add_single_user(pool: web::Data<Pool>,item: web::Json<InputUser>)->Result<usize,diesel::result::Error>{
    let conn = pool.get().unwrap();
    let new_user = NewUser{
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


/**************************************************************************/
#[derive(Debug,Deserialize,Serialize)]
pub struct TokenUser{
    pub name: String,
    pub email: String,
    pub token: Option<String>
}

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    sub: String,
    exp: usize,
}

pub async fn token(query_user: web::Query<TokenUser>)->Result<HttpResponse,Error>{
    use jsonwebtoken::{Header,EncodingKey,encode};

    let my_claims = Claims{
        sub: "b@b.com".to_string(),
        exp: 1601878566
    };

    let token = encode(&Header::default(), &my_claims, &EncodingKey::from_secret("secret".as_ref())).unwrap();

    Ok(
        HttpResponse::Ok()
            .json(
                TokenUser {
                    name: query_user.name.to_string(),
                    email: query_user.email.to_string(),
                    token: Some(token)
                }
            )
    )
}

/**************************************************************************/
pub async fn valid_token(req:HttpRequest)->Result<HttpResponse,RDAError>{
    use jsonwebtoken::{decode, DecodingKey, Validation};
    let key = b"secret";
    let token = req.headers().get("Authorization").unwrap().to_str().unwrap();

    let validation = Validation { sub: Some("b@b.com".to_string()), ..Validation::default() };
    let token_data = match decode::<Claims>(&token,&DecodingKey::from_secret(key),&validation) {
        Ok(c)=>c,
        Err(_)=>panic!("Some error")
    };

    Ok(
        HttpResponse::Ok().json(token_data.claims)
    )
}
