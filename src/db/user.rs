use crate::controllers::auth::SignUpRequest;
use regex::Regex;

pub async fn has_email(db: &sqlx::MySqlPool, email: &str) -> Result<bool, sqlx::Error> {
    let result = sqlx::query("SELECT 1 FROM users WHERE email = ? LIMIT 1")
        .bind(email)
        .fetch_optional(db)
        .await?;

    Ok(result.is_some())
}

pub async fn create_user(
    db: &sqlx::MySqlPool,
    userinfo: &SignUpRequest,
) -> Result<(), sqlx::Error> {
    let email_regex = Regex::new(r"^[^\s@]+@[^\s@]+\.[^\s@]+$").unwrap();
    if !email_regex.is_match(&userinfo.email) {
        return Err(sqlx::Error::Protocol("Invalid email format".into()));
    }

    let encrypted_password = bcrypt::hash(&userinfo.password, bcrypt::DEFAULT_COST)
        .map_err(|_| sqlx::Error::Protocol("Password hashing failed".into()))?;
    sqlx::query!(
        "INSERT INTO users (`first_name`, `last_name`, `email`, `password`) VALUES (?, ?, ?, ?)",
        &userinfo.first_name,
        &userinfo.last_name,
        &userinfo.email,
        &encrypted_password
    )
    .execute(db)
    .await?;
    Ok(())
}
