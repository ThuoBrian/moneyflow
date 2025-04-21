use crate::controllers::auth::SignUpRequest;

pub async fn has_email(db: &sqlx::MySqlPool, email: &str) -> bool {
    sqlx::query("SELECT * FROM users WHERE email = ?")
        .bind(email)
        .fetch_optional(db)
        .await
        .map(|result| result.is_some())
        .unwrap_or(false)
}

pub async fn create_user(
    db: &sqlx::MySqlPool,
    userinfo: &SignUpRequest,
) -> Result<(), sqlx::Error> {
    let validate_email = userinfo.email.contains('@');
    if !validate_email {
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
