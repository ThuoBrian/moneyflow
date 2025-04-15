//To access my project folders
mod controllers;

//To organise my crates
use actix_web::{App, HttpServer, web};
use dotenvy::dotenv;
use std::env;
use tokio::sync::Mutex;

pub struct AppState {
    db: Mutex<sqlx::MySqlPool>,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Load environment variables from .env file
    // Why? it helps to keep sensitive data out of your codebase.
    dotenv().ok();

    let database_url = env::var("DATABASE_URL")
        .expect("You must set the DB config path must be set `DATABASE_URL`");

    let pool = sqlx::MySqlPool::connect(&database_url)
        .await
        .expect("Failed to connect to database, check your connection string");

    let state = web::Data::new(AppState {
        db: Mutex::new(pool),
    });

    println!("Server running at http://127.0.0.1:8080");

    HttpServer::new(move || {
        App::new()
            .app_data(state.clone())
            .service(controllers::auth::sign_in)
            .service(controllers::auth::sign_up)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
