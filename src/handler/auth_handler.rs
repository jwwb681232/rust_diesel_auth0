use crate::schema::users::dsl::*;
use crate::diesel::RunQueryDsl;
use crate::rim::error::{ApiResult, ApiError};

use actix_web::{web, HttpResponse};
use diesel::dsl::{insert_into};

use crate::Pool;
use crate::model::auth_model::{RegisterForm, InsertRegisterData};

pub async fn register(db: web::Data<Pool>, item: web::Form<RegisterForm>) -> ApiResult<HttpResponse> {
    Ok(
        web::block(move || add_user(db, item))
            .await
            .map(|inserted| HttpResponse::Ok().json(inserted))
            .map_err(|_| ApiError::new(500, "Internal Server Error"))?
    )
}

fn add_user(pool: web::Data<Pool>, item: web::Form<RegisterForm>) -> Result<usize, diesel::result::Error> {
    let conn = pool.get().unwrap();

    let new_user = InsertRegisterData {
        first_name: item.first_name.to_string(),
        last_name: item.last_name.to_string(),
        email: item.email.to_string(),
        password: item.password.to_string(),
        updated_at: chrono::Local::now().naive_local(),
    };

    Ok(
        insert_into(users).values(&new_user).execute(&conn)?
    )
}
