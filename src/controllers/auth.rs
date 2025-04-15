use crate::AppState;
use actix_web::{HttpResponse, Responder, get, post, web};
use serde::{Deserialize, Serialize};
use sqlx::MySqlPool;
use std::error::Error;

#[derive(Deserialize, Debug)]
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
pub async fn sign_up(data: web::Json<SignUpRequest>) -> impl Responder {
    HttpResponse::Ok().json(SignUpResponse {
        id: 1, // Placeholder ID
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
