use actix_web::{web, FromRequest};
use crate::handler::auth_handler;
use crate::extractor::error_handler::json_error_handler;
use crate::model::user_model::RegisterUser;

pub fn route(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/auth")
            .service(
                web::resource("/register")
                    .app_data(web::Json::<RegisterUser>::configure(|json_config| {
                        json_config.error_handler(json_error_handler)
                    }))
                    .route(web::post().to(auth_handler::register))
            )
    );
}
