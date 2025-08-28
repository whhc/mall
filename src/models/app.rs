use axum::extract::FromRef;
use sea_orm::DatabaseConnection;
use serde::Serialize;
use std::sync::Arc;

#[derive(Clone, Debug)]
pub struct AppState {
    pub db: DatabaseConnection,
    pub jwt_secret: String,
}

// 实现FromRef，使得可以从AppState中提取Arc<DatabaseConnection>
impl FromRef<AppState> for Arc<DatabaseConnection> {
    fn from_ref(app_state: &AppState) -> Arc<DatabaseConnection> {
        Arc::new(app_state.db.clone())
    }
}

pub struct _AppState {
    db: DatabaseConnection,
    jwt_secret: String,
}

/// 统一的API响应结构
#[derive(Debug, Serialize)]
pub struct ApiResponse<T> {
    pub code: u16,
    pub message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<T>,
}

impl<T> ApiResponse<T> {
    pub fn success(data: T) -> Self {
        Self {
            code: 200,
            message: "成功".to_string(),
            data: Some(data),
        }
    }

    pub fn success_with_message(data: T, message: &str) -> Self {
        Self {
            code: 200,
            message: message.to_string(),
            data: Some(data),
        }
    }
}

impl ApiResponse<()> {
    pub fn error(message: &str) -> Self {
        Self {
            code: 400,
            message: message.to_string(),
            data: None,
        }
    }

    pub fn error_with_code(code: u16, message: &str) -> Self {
        Self {
            code,
            message: message.to_string(),
            data: None,
        }
    }
}
