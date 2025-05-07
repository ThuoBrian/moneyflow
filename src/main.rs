//To access my project folders
mod controllers;
mod db;

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

    let database_url = env::var("DATABASE_URL").expect("Check your db path config `DATABASE_URL`");

    let state = web::Data::new(AppState {
        db: Mutex::new(sqlx::MySqlPool::connect(&database_url).await.unwrap()),
    });

    println!("\n Server running at http://127.0.0.1:8080 \n");

    HttpServer::new(move || {
        App::new()
            .app_data(state.clone())
            .service(controllers::auth::sign_in)
            .service(controllers::auth::sign_up)
            .service(controllers::me::get_profile)
            .service(controllers::me::update_user_profile)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
