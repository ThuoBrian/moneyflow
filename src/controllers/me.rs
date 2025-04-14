use actix_web::{Responder, get, post};

#[get("/me")]
pub async fn get_profile() -> impl Responder {
    "Profile"
}

#[get("/me")]
pub async fn get_user_requests() -> impl Responder {
    "All requests"
}

#[get("/me")]
pub async fn get_user_activation_status() -> impl Responder {
    "is user activate"
}

#[post("/me")]
pub async fn update_profile() -> impl Responder {
    "Update Profile"
}

#[post("/me")]
pub async fn query_user_transactions() -> impl Responder {
    " Transactions: "
}

#[post("/me")]
pub async fn query_user_pending_items() -> impl Responder {
    "Categories"
}

#[post("/me")]
pub async fn query_user_available_balance() -> impl Responder {
    "Available Balances"
}
