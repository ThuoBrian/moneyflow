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
    // const MIN_PASSWORD_LENGTH: usize = 10;
    // const SPECIAL_CHARS: &str = "!@#$%^&*()_+-=[]{}|;':\",.<>?/`~";

    // let password = &userinfo.password;

    // if password.len() < MIN_PASSWORD_LENGTH {
    //     return Err(sqlx::Error::Protocol(
    //         "Password Must be 10 char and over".into(),
    //     ));
    // }

    // // Single pass to check all character requirements
    // let mut has_uppercase = false;
    // let mut has_lowercase = false;
    // let mut has_digit = false;
    // let mut has_special = false;

    // for c in password.chars() {
    //     if c.is_whitespace() {
    //         return Err(sqlx::Error::Protocol(
    //             "Password must not contain any whitespace characters".into(),
    //         ));
    //     }
    //     if c.is_uppercase() {
    //         has_uppercase = true;
    //     } else if c.is_lowercase() {
    //         has_lowercase = true;
    //     }
    //     if c.is_digit(10) {
    //         has_digit = true;
    //     }
    //     if SPECIAL_CHARS.contains(c) {
    //         has_special = true;
    //     }
    // }

    // // Check all requirements in one go
    // if !has_uppercase {
    //     return Err(sqlx::Error::Protocol(
    //         "Password must contain at least one uppercase letter".to_string(),
    //     ));
    // }
    // if !has_lowercase {
    //     return Err(sqlx::Error::Protocol(
    //         "Password must contain at least one lowercase letter".to_string(),
    //     ));
    // }
    // if !has_digit {
    //     return Err(sqlx::Error::Protocol(
    //         "Password must contain at least one digit".to_string(),
    //     ));
    // }
    // if !has_special {
    //     return Err(sqlx::Error::Protocol(
    //         "Password must contain at least one special character".to_string(),
    //     ));
    // }

    // Encrypt the password only after all validations pass
    let encrypted_password = bcrypt::hash(&userinfo.password, bcrypt::DEFAULT_COST)
        .map_err(|_| sqlx::Error::Protocol("Password hashing failed".into()))?;

    // Insert into the database only after all validations and encryption succeed
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
