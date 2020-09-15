use actix_web::{web, FromRequest};
use crate::handler::auth_handler;
use crate::extractor::error_handler::form_error_handler;
use crate::model::auth_model::RegisterForm;

pub fn route(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/auth")
            .service(
                web::resource("/register")
                    .app_data(web::Form::<RegisterForm>::configure(|form_config| {
                        form_config.error_handler(form_error_handler)
                    }))
                    .route(web::post().to(auth_handler::register))
            )
    );
}
