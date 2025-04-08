use actix_web::{web, App, HttpServer};
use diesel::r2d2::{self, ConnectionManager};
use diesel::PgConnection;
use std::env;
use dotenvy::dotenv;

mod schema;
mod models;
mod routes;

type DbPool = r2d2::Pool<ConnectionManager<PgConnection>>;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    //dotenv().ok();
    // Read DATABASE_URL from .env
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    // Set up connection pool
    let manager = ConnectionManager::<PgConnection>::new(database_url);
    let pool = r2d2::Pool::builder().build(manager).expect("Failed to create pool");

    println!("Starting server at http://0.0.0.0:8080");

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .service(routes::create_user)
            .service(routes::get_users)
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await
}
