// src/main.rs

use actix_cors::Cors;
use actix_web::{middleware, web, App, HttpServer};
use dotenv::dotenv;
use std::env;

mod chat_router;
mod chat_model;
mod crypto_enc;
mod db_mysql;
mod socket_handlers;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Load environment variables
    dotenv().ok();
    env_logger::init();

    let auth_header_password =
        env::var("AUTHHEADER_PASSWORD").expect("AUTHHEADER_PASSWORD must be set");
    let cors_origin = env::var("CORS_ORIGIN").expect("CORS_ORIGIN must be set");

    // Create database connection pool
    let pool = db_mysql::create_pool();

    // Shared state
    let shared_data = web::Data::new(chat_model::AppState {
        room_users: std::sync::Mutex::new(Vec::new()),
        pool: pool.clone(),
    });

    // Start HTTP server
    HttpServer::new(move || {
        let cors = Cors::default()
            .allowed_origin(&cors_origin)
            .allowed_methods(vec!["GET", "POST", "OPTIONS"])
            .allowed_headers(vec![
                actix_web::http::header::AUTHORIZATION,
                actix_web::http::header::ACCEPT,
                actix_web::http::header::CONTENT_TYPE,
            ])
            .supports_credentials()
            .max_age(3600);

        App::new()
            .wrap(middleware::Logger::default())
            .wrap(cors)
            .app_data(shared_data.clone())
            .app_data(web::Data::new(auth_header_password.clone()))
            .configure(chat_router::init_routes)
            .service(actix_files::Files::new("/", "./static").index_file("index.html"))
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}