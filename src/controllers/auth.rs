use crate::{AppState, db};
use actix_web::{
    HttpResponse, Responder, post,
    web::{self},
};
use regex::Regex;
use serde::{Deserialize, Serialize};

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
        return HttpResponse::BadRequest().body("email already exists");
    }

    let email_regex = Regex::new(r"^[^\s@]+@[^\s@]+\.[^\s@]+$").unwrap();
    if !email_regex.is_match(&data.email) {
        return HttpResponse::BadRequest().body("Invalid email format - Weka email poa");
    }

    db::user::create_user(&db, &data).await.ok();
    "Success, User Information is added ".to_string();

    HttpResponse::Ok().json(SignUpResponse {
        id: 1, // Replace with actual ID generation logic
        first_name: data.first_name.clone(),
        last_name: data.last_name.clone(),
        email: data.email.clone(),
        password: data.password.clone(),
    })
}

#[post("/auth/signin")]
pub async fn sign_in(data: web::Json<SignUpRequest>) -> impl Responder {
    HttpResponse::Ok().json(SignUpResponse {
        id: 1,
        first_name: data.first_name.clone(),
        last_name: data.last_name.clone(),
        email: data.email.clone(),
        password: data.password.clone(),
    })
}
