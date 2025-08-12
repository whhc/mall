use std::sync::Arc;

use anyhow::{Result, anyhow};
use bcrypt::{DEFAULT_COST, hash};
use chrono::{DateTime, Utc};
use sea_orm::{DatabaseConnection, EntityTrait, QueryFilter, entity::*};

use entity::user;

use crate::middlewares::auth::AuthenticatedUser;

use super::email::{EmailConfig, generate_verification_code, send_verification_email};

pub async fn register(
    db: Arc<DatabaseConnection>,
    email: String,
    password: String,
    name: String,
) -> Result<user::Model> {
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

    let email_verification_token = generate_verification_code();
    let now: DateTime<Utc> = Utc::now();

    let new_user = user::ActiveModel {
        email: Set(email),
        password: Set(password_hash),
        name: Set(name),
        email_verification_token: Set(Some(email_verification_token.clone())),
        email_verification_sent_at: Set(Some(now)),
        ..Default::default()
    };

    let email_config = EmailConfig::from_env()?;
    // 发送验证码
    let _ = send_verification_email(
        &email_config,
        "Gao Guixi <gao.guixi@qq.com>".to_string(),
        &email_verification_token,
        &super::email::CodeType::REGISTER,
    )
    .await?;

    let user = new_user
        .insert(&*db)
        .await
        .map_err(|e| anyhow!("Insert user to database error: {}", e))?;

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
    _auth_user: AuthenticatedUser,
) -> Result<user::Model> {
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
