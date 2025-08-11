use std::sync::Arc;

use anyhow::{Result, anyhow};
use bcrypt::{DEFAULT_COST, hash};
use chrono::{DateTime, Utc};
use sea_orm::{DatabaseConnection, EntityTrait, QueryFilter, entity::*};

use entity::user;
use uuid::Uuid;

use crate::middlewares::auth::AuthenticatedUser;

pub async fn register(
    db: Arc<DatabaseConnection>,
    email: String,
    password: String,
    name: String,
) -> Result<user::Model> {
    tracing::info!("Register start");
    let existing_user = user::Entity::find()
        .filter(user::Column::Email.eq(&email))
        .one(&*db)
        .await
        .map_err(|e| anyhow!("Database query error: {}", e))?;

    if existing_user.is_some() {
        return Err(anyhow!("This email has registered"));
    }

    let password_hash =
        hash(&password, DEFAULT_COST).map_err(|e| anyhow!("Password hash error: {}", e))?;

    let email_verification_token = Uuid::new_v4().to_string();
    let now: DateTime<Utc> = Utc::now();

    println!("Email verification sent at: {}", now);
    tracing::info!("Email verification sent at: {}", now);

    let new_user = user::ActiveModel {
        email: Set(email),
        password: Set(password_hash),
        name: Set(name),
        email_verification_token: Set(Some(email_verification_token)),
        email_verification_sent_at: Set(Some(now)),
        ..Default::default()
    };

    let user = new_user
        .insert(&*db)
        .await
        .map_err(|e| anyhow!("Insert user to database error: {}", e))?;

    // 发送验证码
    // send_verification_email(&user).await?;

    tracing::info!("Register success");
    Ok(user)
}

pub async fn _verify_email(db: Arc<DatabaseConnection>, token: &str) -> Result<user::Model> {
    let select_user = user::Entity::find()
        .filter(user::Column::EmailVerificationToken.eq(Some(token.to_string())))
        .one(&*db)
        .await
        .map_err(|e| anyhow!("Database query error: {}", e))?
        .ok_or_else(|| anyhow!("Invalid email token"))?;

    let mut user_active_model: user::ActiveModel = select_user.into();
    user_active_model.email_verified_at = Set(Some(chrono::Utc::now()));
    user_active_model.email_verification_token = Set(None);
    user_active_model.email_verification_sent_at = Set(None);

    let updated_user = user_active_model
        .update(&*db)
        .await
        .map_err(|e| anyhow!("User updated error: {}", e))?;

    Ok(updated_user)
}

pub async fn get_user(
    db: Arc<DatabaseConnection>,
    user_id: i32,
    auth_user: AuthenticatedUser,
) -> Result<user::Model> {
    tracing::info!("Auth_user: {auth_user:?}");

    let existing_user = user::Entity::find()
        .filter(user::Column::Id.eq(user_id))
        .one(&*db)
        .await
        .map_err(|e| anyhow!("Database query error: {}", e))?;

    if existing_user.is_none() {
        return Err(anyhow!("Has not user"));
    }

    let user = existing_user.unwrap();
    Ok(user)
}
