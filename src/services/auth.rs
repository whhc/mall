use anyhow::{Result, anyhow};
use bcrypt::verify;
use chrono::{Duration, Utc};
use entity::user;
use jsonwebtoken::{Algorithm, EncodingKey, Header, encode};
use sea_orm::{DatabaseConnection, EntityTrait, QueryFilter, entity::*};

use crate::models::dtos::LoginResponse;

pub async fn login(db: &DatabaseConnection, email: &str, password: &str) -> Result<LoginResponse> {
    let existing_user = user::Entity::find()
        .filter(user::Column::Email.eq(email))
        .one(db)
        .await
        .map_err(|e| anyhow!("Email is not exist: {}", e))?;

    if let Some(user) = existing_user {
        if verify(password, &user.password)? {
            let secret = std::env::var("JWT_SECRET")?;
            let expiry_hours = std::env::var("JWT_EXPIRATION_HOURS")?;
            let expiry = Utc::now() + Duration::hours(expiry_hours.parse()?);
            let claims = Claims {
                sub: user.id.to_string(),
                exp: expiry.timestamp() as usize,
            };
            let token = encode(
                &Header::new(Algorithm::HS256),
                &claims,
                &EncodingKey::from_secret(secret.as_bytes()),
            )?;

            Ok(LoginResponse {
                token,
                user_id: user.id.to_string(),
            })
        } else {
            return Err(anyhow!("Password error"));
        }
    } else {
        return Err(anyhow!("User is not exist"));
    }
}

#[derive(serde::Serialize, serde::Deserialize)]
struct Claims {
    sub: String,
    exp: usize,
}
