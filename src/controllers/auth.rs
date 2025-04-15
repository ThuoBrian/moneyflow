use crate::{AppState, db};
use actix_web::{
    HttpResponse, Responder, get, post,
    web::{self, Data},
};
use serde::{Deserialize, Serialize};
use sqlx::MySqlPool;
use std::error::Error;

#[derive(Serialize, Deserialize, Debug)]
pub struct SignUpRequest {
    first_name: String,
    last_name: String,
    email: String,
    password: String,
}

#[derive(Serialize)]
struct SignUpResponse {
    id: u64,
    first_name: String,
    last_name: String,
    email: String,
    password: String,
}

#[post("/auth/sign-up")]
pub async fn sign_up(state: web::Data<AppState>, data: web::Json<SignUpRequest>) -> impl Responder {
    let db = state.db.lock().await;

    if db::user::has_email(&db, &data.email).await {
        return HttpResponse::BadRequest().body("Email already exists");
    }

    HttpResponse::Ok().json(SignUpRequest {
        first_name: data.first_name.clone(),
        last_name: data.last_name.clone(),
        email: data.email.clone(),
        password: data.password.clone(),
    })
}

#[post("/auth/sign-in")]
pub async fn sign_in(data: web::Json<SignUpRequest>) -> impl Responder {
    HttpResponse::Ok().json(SignUpResponse {
        id: 1,
        first_name: data.first_name.clone(),
        last_name: data.last_name.clone(),
        email: data.email.clone(),
        password: data.password.clone(),
    })
}
