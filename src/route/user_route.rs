use actix_web::{web, FromRequest};
use crate::handler::user_handler;
use crate::extractor::error_handler::json_error_handler;
use crate::model::user_model::InputUser;

pub fn route(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/users")
            .route("", web::get().to(user_handler::get_users))
            .route("{id}", web::get().to(user_handler::get_user_by_id))
            .route("{id}", web::delete().to(user_handler::delete_user))
            .service(
                web::resource("")
                    .app_data(web::Json::<InputUser>::configure(|json_config| {
                        json_config.error_handler(json_error_handler)
                    }))
                    .route(web::post().to(user_handler::add_user))
            )
    );
}
