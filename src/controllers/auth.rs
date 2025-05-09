use crate::{AppState, db};
use actix_web::{
    HttpResponse, Responder, post,
    web::{self},
};
use regex::Regex;
use serde::{Deserialize, Serialize};
use serde_json::json;

#[derive(Serialize, Deserialize, Debug)]
pub struct SignUpRequest {
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub password: String,
}

#[derive(Serialize)]
pub struct SignUpResponse {
    id: u64,
    first_name: String,
    last_name: String,
    email: String,
    password: String,
}

#[post("/auth/signup")]
pub async fn sign_up(state: web::Data<AppState>, data: web::Json<SignUpRequest>) -> impl Responder {
    let db = state.db.lock().await;

    if db::user::has_email(&db, &data.email)
        .await
        .expect("Database query failed")
    {
        return HttpResponse::UnprocessableEntity().json(json!({
            "STATUS": "Error",
            "Message": "Email already exists."
        }));
    }

    let email_regex = Regex::new(r"^[^\s@]+@[^\s@]+\.[^\s@]+$").unwrap();
    if !email_regex.is_match(&data.email) {
        return HttpResponse::UnprocessableEntity().json(json!({
            "STATUS": "Error",
            "Message": "Invalid email format"
        }));
    }

    if data.password.len() < 8 {
        return HttpResponse::UnprocessableEntity().json(json!({
            "STATUS": "Error",
            "Message": "Password must be at least 8 characters long"
        }));
    }

    match db::user::create_user(&db, &data).await {
        Ok(()) => {
            HttpResponse::Created().json(SignUpResponse {
                id: 1,
                first_name: data.first_name.clone(),
                last_name: data.last_name.clone(),
                email: data.email.clone(),
                password: data.password.clone(), // ideally don't return password
            })
        }
        Err(e) => {
            eprintln!("Error creating user: {:?}", e);
            HttpResponse::UnprocessableEntity().json(json!({
                "STATUS": "Error",
                "Message": "Failed to create user"
            }))
        }
    }
}
#[post("/auth/signin")]

pub async fn sign_in() -> impl Responder {
    "Sign in endpoint"
}
