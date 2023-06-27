pub mod models;
pub mod routes;
pub mod schema;

use actix_web::middleware::Logger;
use actix_web::{web, App, HttpServer};
use diesel::r2d2::ConnectionManager;
use diesel::r2d2::Pool;
use diesel::SqliteConnection;

pub type ConnectionPool = Pool<ConnectionManager<SqliteConnection>>;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Configure logging.
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    // Use connection pool.
    let manager = ConnectionManager::<SqliteConnection>::new("checkpoint.db");
    let pool = Pool::builder()
        .build(manager)
        .expect("Database URL should be valid path to SQLite DB file.");

    // Return an Actix server.
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .service(routes::signup::signup)
            .service(routes::token::token)
            .wrap(Logger::default())
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
