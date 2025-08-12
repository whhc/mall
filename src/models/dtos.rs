use serde::{Deserialize, Serialize};

/// 用户注册请求DTO
#[derive(Debug, Deserialize)]
pub struct RegisterUserDto {
    pub email: String,
    pub password: String,
    pub name: String,
}

/// 用户注册响应DTO
#[derive(Debug)]
pub struct _RegisterResponse {
    pub message: String,
    pub user_id: i32,
    pub email: String,
    pub email_verified: bool,
}

#[derive(Debug, Deserialize)]
pub struct LoginUserDto {
    pub email: String,
    pub password: String,
}

#[derive(Debug, Serialize)]
pub struct LoginResponse {
    pub user_id: String,
    pub token: String,
}
