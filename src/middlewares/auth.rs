use std::sync::Arc;

use axum::{
    extract::{FromRef, FromRequestParts},
    http::{StatusCode, request::Parts},
};
use entity::user::{Entity as User, UserRole};
use jsonwebtoken::{DecodingKey, Validation, decode};
use sea_orm::ColumnTrait;
use sea_orm::{DatabaseConnection, EntityTrait, QueryFilter};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone)]
pub struct AuthenticatedUser {
    pub user_id: String,
    pub role: UserRole,
}

impl AuthenticatedUser {
    /// 检查用户是否为管理员
    pub fn is_admin(&self) -> bool {
        matches!(self.role, UserRole::Admin)
    }

    /// 检查用户是否可以访问下架产品
    pub fn can_access_inactive_products(&self) -> bool {
        self.is_admin()
    }

    /// 检查用户是否可以修改产品
    pub fn can_modify_products(&self) -> bool {
        self.is_admin()
    }

    /// 检查用户是否可以管理分类和地区
    pub fn can_manage_categories_regions(&self) -> bool {
        self.is_admin()
    }
}

impl<S> FromRequestParts<S> for AuthenticatedUser
where
    Arc<DatabaseConnection>: FromRef<S>,
    S: Send + Sync + std::fmt::Debug,
{
    type Rejection = (StatusCode, String);

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        let db = Arc::<DatabaseConnection>::from_ref(state);

        let auth_header = parts
            .headers
            .get("Authorization")
            .ok_or((
                StatusCode::UNAUTHORIZED,
                "Authorization is missing".to_string(),
            ))?
            .to_str()
            .map_err(|_| {
                (
                    StatusCode::UNAUTHORIZED,
                    "Invalid Authorization header".to_string(),
                )
            })?;

        let cc = parts
            .headers
            .get("Cc")
            .ok_or((
                StatusCode::NON_AUTHORITATIVE_INFORMATION,
                "Cc is missing".to_string(),
            ))?
            .to_str()
            .map_err(|_| {
                (
                    StatusCode::NON_AUTHORITATIVE_INFORMATION,
                    "Invalid cc header".to_string(),
                )
            })?;

        let token = auth_header.strip_prefix("Bearer ").ok_or((
            StatusCode::UNAUTHORIZED,
            "Authorization header format must be 'Bearer <token>'".to_string(),
        ))?;

        let secret = std::env::var("JWT_SECRET").map_err(|_| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                "JWT secret not configured".to_string(),
            )
        })?;

        let validation = Validation::new(jsonwebtoken::Algorithm::HS256);
        let decoded = decode::<Claims>(
            token,
            &DecodingKey::from_secret(secret.as_bytes()),
            &validation,
        )
        .map_err(|_| {
            (
                StatusCode::UNAUTHORIZED,
                "Invalid or expired token".to_string(),
            )
        })?;

        let decoded_cc = decoded.claims.sub.clone();

        if &decoded_cc != cc {
            return Err((
                StatusCode::UNAUTHORIZED,
                "Invalid Authorization info ".to_string(),
            ));
        }

        let user = User::find()
            .filter(
                entity::user::Column::Id.eq(decoded.claims.sub.parse::<i32>().map_err(|_| {
                    (
                        StatusCode::UNAUTHORIZED,
                        "Invalid user ID in token".to_string(),
                    )
                })?),
            )
            .one(db.as_ref())
            .await
            .map_err(|e| {
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    format!("Middleware auth database error when get user {e}"),
                )
            })?
            .ok_or((StatusCode::UNAUTHORIZED, "User not found".to_string()))?;

        let role = user.role;

        tracing::info!("User {cc} login with role {role:?}");

        Ok(AuthenticatedUser {
            user_id: cc.to_string(),
            role,
        })
    }
}

/// 可选的认证用户 - 用于允许匿名访问但需要区分用户状态的接口
pub struct OptionalAuthenticatedUser(pub Option<AuthenticatedUser>);

impl<S> FromRequestParts<S> for OptionalAuthenticatedUser
where
    Arc<DatabaseConnection>: FromRef<S>,
    S: Send + Sync + std::fmt::Debug,
{
    type Rejection = (StatusCode, String);

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        // 尝试获取认证用户，如果失败则返回None
        match AuthenticatedUser::from_request_parts(parts, state).await {
            Ok(user) => Ok(OptionalAuthenticatedUser(Some(user))),
            Err(_) => Ok(OptionalAuthenticatedUser(None)),
        }
    }
}

#[derive(Deserialize, Serialize)]
struct Claims {
    sub: String,
    exp: Option<usize>,
}
