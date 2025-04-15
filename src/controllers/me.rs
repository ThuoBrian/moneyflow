use actix_web::{Responder, get, post};

#[get("/me")]
pub async fn get_profile() -> impl Responder {
    "Profile"
}

#[post("/me")]
pub async fn update_user_profile() -> impl Responder {
    "Update User Profile"
}
