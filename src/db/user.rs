use crate::controllers::auth::{SignUpRequest, SignUpResponse};


pub async fn has_email(db: &sqlx::MySqlPool, email: &str) -> bool {
    sqlx::query!("SELECT * FROM users WHERE email = ?", email)
        .fetch_optional(db)
        .await
        .map(|result| result.is_some())
        .unwrap_or(false)
}

pub async fn create_user(db: &sqlx::MySqlPool, userinfo: &SignUpRequest) -> bool {
    let encrypted_password = bcrypt::hash(&userinfo.password, bcrypt::DEFAULT_COST).unwrap();
    sqlx::query!(
        "INSERT INTO users (`first_name`, `last_name`, `email`,`password`) VALUES (?, ?, ?, ?)",
        &userinfo.first_name,
        &userinfo.last_name,
        &userinfo.email,
        &encrypted_password
    )
    .execute(db)
    .await
    .is_ok()
}
