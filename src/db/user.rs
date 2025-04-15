pub async fn has_email(db: &sqlx::MySqlPool, email: &str) -> bool {
    sqlx::query!("SELECT * FROM users WHERE email = ?", email)
        .fetch_optional(db)
        .await
        .map(|result| result.is_some())
        .unwrap_or(false)
}
