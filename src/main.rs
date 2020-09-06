use actix_web::{HttpServer,web,App};

mod handler;

#[actix_rt::main]
async fn main()->std::io::Result<()>{
    HttpServer::new(||{
        App::new()
            .route("/register",web::post().to(handler::auth_handler::index))
    })
        .bind("127.0.0.1:8088")?
        .run()
        .await
}