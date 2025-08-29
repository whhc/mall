use entity::product::ProductStatus;
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
// use entity::product::ProductStatus; // 将在迁移后启用

// // 临时定义，将迁移后替换为 entity::product::ProductStatus
// #[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
// pub enum ProductStatus {
//     Active,
//     Inactive,
// }

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

#[derive(Debug, Deserialize)]
pub struct UpdateUserRequest {
    pub name: String,
}

// ======================== 产品管理相关 DTO ========================

/// 产品列表查询参数
#[derive(Debug, Deserialize)]
pub struct ProductListParams {
    pub page: Option<u64>,
    pub limit: Option<u64>,
    pub category_id: Option<i64>,
    pub status: Option<ProductStatus>,
    pub region_id: Option<i64>,
    pub search: Option<String>,
}

/// 创建产品请求DTO
#[derive(Debug, Deserialize)]
pub struct CreateProductRequest {
    pub product_name: String,
    pub product_description: Option<String>,
    pub product_price: Decimal,
    pub product_stock: i32,
    pub status: ProductStatus,
    pub product_image: Option<String>,
    pub category_ids: Vec<i64>,
    pub region_ids: Vec<i64>,
}

/// 更新产品请求DTO
#[derive(Debug, Deserialize)]
pub struct UpdateProductRequest {
    pub product_name: Option<String>,
    pub product_description: Option<String>,
    pub product_price: Option<Decimal>,
    pub product_stock: Option<i32>,
    pub status: Option<ProductStatus>,
    pub product_image: Option<String>,
    pub category_ids: Option<Vec<i64>>,
    pub region_ids: Option<Vec<i64>>,
}

/// 分类信息
#[derive(Debug, Serialize, Deserialize)]
pub struct CategoryInfo {
    pub category_id: i64,
    pub category_name: String,
    pub parent_category_id: Option<i64>,
}

/// 地区信息
#[derive(Debug, Serialize, Deserialize)]
pub struct RegionInfo {
    pub region_id: i64,
    pub region_name: String,
}

/// 产品响应DTO
#[derive(Debug, Serialize)]
pub struct ProductResponse {
    pub product_id: i64,
    pub product_name: String,
    pub product_description: Option<String>,
    pub product_price: f64,
    pub product_stock: i32,
    pub status: ProductStatus,
    pub product_image: Option<String>,
    pub categories: Vec<CategoryInfo>,
    pub regions: Vec<RegionInfo>,
    pub created_at: String,
    pub updated_at: String,
}

/// 产品列表响应DTO
#[derive(Debug, Serialize)]
pub struct ProductListResponse {
    pub products: Vec<ProductResponse>,
    pub total: u64,
    pub page: u64,
    pub limit: u64,
}

/// 创建分类请求DTO
#[derive(Debug, Deserialize)]
pub struct CreateCategoryRequest {
    pub category_name: String,
    pub parent_category_id: Option<i64>,
}

/// 创建地区请求DTO
#[derive(Debug, Deserialize)]
pub struct CreateRegionRequest {
    pub region_name: String,
}

/// 分类列表响应DTO
#[derive(Debug, Serialize)]
pub struct CategoryListResponse {
    pub categories: Vec<CategoryInfo>,
}

/// 地区列表响应DTO
#[derive(Debug, Serialize)]
pub struct RegionListResponse {
    pub regions: Vec<RegionInfo>,
}
