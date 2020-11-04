#[macro_use]
extern crate diesel;

use actix_web::{middleware::Logger, App, HttpServer};
use diesel::prelude::*;
use diesel::r2d2::{self, ConnectionManager};

mod handler;
mod model;
mod rim;
mod route;
mod schema;

pub type Pool = r2d2::Pool<ConnectionManager<MysqlConnection>>;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();
    std::env::set_var("RUST_LOG", "actix_web=debug");

    env_logger::init();

    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let manager = ConnectionManager::<MysqlConnection>::new(database_url);
    let pool = r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create pool.");

    HttpServer::new(move || {
        App::new()
            .wrap(Logger::new("%a %r %s %t"))
            .data(pool.clone())
            .configure(route::user_route::route)
    })
    .bind("127.0.0.1:8888")?
    .run()
    .await
}
