use crate::{AppState, db};
use actix_web::{
    HttpResponse, Responder, post,
    web::{self},
};
use bcrypt::verify;
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
            "MESSAGE": "Email already exists."
        }));
    }

    let email_regex: Regex = Regex::new(r"^[^\s@]+@[^\s@]+\.[^\s@]+$").unwrap();
    if !email_regex.is_match(&data.email) {
        return HttpResponse::UnprocessableEntity().json(json!({
            "STATUS": "Error",
            "MESSAGE": "Invalid email format"
        }));
    }

    if data.password.len() < 8 {
        return HttpResponse::UnprocessableEntity().json(json!({
            "STATUS": "Error",
            "MESSAGE": "Password must be at least 8 characters long"
        }));
    }

    match db::user::create_user(&db, &data).await {
        Ok(()) => HttpResponse::Created().json(json!({
            "Status" : "Success",
            "MESSAGE" : "User is created, Successfully"
        })),
        Err(e) => {
            eprintln!("Error creating user: {:?}", e);
            HttpResponse::UnprocessableEntity().json(json!({
                "STATUS": "Error",
                "MESSAGE": "Failed to create user"
            }))
        }
    }
}

#[derive(Deserialize, Debug)]
pub struct SignInRequest {
    pub email: String,
    pub password: String,
}

#[post("/auth/signin")]
pub async fn sign_in(state: web::Data<AppState>, data: web::Json<SignInRequest>) -> impl Responder {
    let db = state.db.lock().await;
    let user = db::user::get_user_by_email(&db, data.email.clone()).await;

    if user.is_none() {
        return HttpResponse::Unauthorized().json(json!({
            "STATUS": "Error",
            "MESSAGE": "Invalid email or Password"
        }));
    }

    let user = user.unwrap();
    if !bcrypt::verify(&data.password, &user.password).unwrap() {
        return HttpResponse::Unauthorized().json(json!({
            "STATUS": "Error",
            "MESSAGE": "Invalid email or Password"
        }));
    }

    HttpResponse::Ok().json(json!({
        "STATUS": "Success",
        "MESSAGE": "Sign in successful"
    }))
}

// #[get("/auth/getuser/{id}")]
// pub async fn get_user_email(state: web::Data<AppState>, id: web::Path<u64>) -> impl Responder {
//     let db = state.db.lock().await;
//     // to create a get user endpoint

//     let user = db::user::get_user_by_email(&db, &email).await;
//     match user {
//         Ok(user) => HttpResponse::Ok().json(user),
//         Err(e) => {
//             eprintln!("Error fetching user: {:?}", e);
//             HttpResponse::UnprocessableEntity().finish()
//         }
//     }
// }
