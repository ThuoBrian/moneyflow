use crate::controllers::auth::SignUpRequest;
use regex::Regex;
use sqlx::types::chrono;

pub async fn has_email(db: &sqlx::MySqlPool, email: &str) -> Result<bool, sqlx::Error> {
    let result = sqlx::query("SELECT 1 FROM users WHERE email = ? LIMIT 1")
        .bind(email)
        .fetch_optional(db)
        .await?;

    Ok(result.is_some())
}

pub async fn does_username_exist(
    db: &sqlx::MySqlPool,
    f_name: &str,
    l_name: &str,
) -> Result<bool, sqlx::Error> {
    let result = sqlx::query("SELECT 1 FROM users WHERE first_name = ? AND last_name = ? LIMIT 1")
        .bind(f_name)
        .bind(l_name)
        .fetch_optional(db)
        .await?;

    if result.is_some() {
        return Err(sqlx::Error::Protocol("Username already exists".into()));
    }
    Ok(false)
}

pub async fn create_user(
    db: &sqlx::MySqlPool,
    userinfo: &SignUpRequest,
) -> Result<(), sqlx::Error> {
    // this regex checks for a valid email format.
    const EMAIL_FORMAT_CHECK: &str = r"^[^\s@]+@[^\s@]+\.[^\s@]+$";

    let email_regex = Regex::new(EMAIL_FORMAT_CHECK).unwrap();
    if !email_regex.is_match(&userinfo.email) {
        return Err(sqlx::Error::Protocol("Invalid email format".into()));
    }
    if *&userinfo.password.len() < 8 {
        return Err(sqlx::Error::Protocol(
            "Password must be at least 8 characters long".into(),
        ));
    }
    let password = &userinfo.password;

    // Single pass to check all character requirements
    let mut has_uppercase = false;
    let mut has_lowercase = false;
    let mut has_digit = false;

    for c in password.chars() {
        if c.is_whitespace() {
            return Err(sqlx::Error::Protocol(
                "Password must not contain any whitespace characters".into(),
            ));
        }
        if c.is_uppercase() {
            has_uppercase = true;
        } else if c.is_lowercase() {
            has_lowercase = true;
        }
        if c.is_digit(10) {
            has_digit = true;
        }
    }

    // Check all requirements in one go
    if !has_uppercase {
        return Err(sqlx::Error::Protocol(
            "Password must contain at least one uppercase letter".to_string(),
        ));
    }
    if !has_lowercase {
        return Err(sqlx::Error::Protocol(
            "Password must contain at least one lowercase letter".to_string(),
        ));
    }
    if !has_digit {
        return Err(sqlx::Error::Protocol(
            "Password must contain at least one digit".to_string(),
        ));
    }

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

#[warn(dead_code)]
pub struct User {
    pub id: u64,
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub password: String,
    pub balance: u64,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
}

pub async fn get_user_by_email(db: &sqlx::MySqlPool, user_email: String) -> Option<User> {
    sqlx::query_as!(User, "SELECT * FROM users WHERE  email = ?", user_email)
        .fetch_optional(db)
        .await
        .unwrap()
}
