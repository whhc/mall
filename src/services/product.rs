use ::entity::{product, product_category, product_region};
use anyhow::{Ok, Result, anyhow};
use chrono::Utc;
use sea_orm::*;

use crate::middlewares::auth::AuthenticatedUser;
use crate::models::dtos::*;

pub async fn list_products(
    db: &DatabaseConnection,
    params: ProductListParams,
    _user: Option<&AuthenticatedUser>,
) -> Result<ProductListResponse> {
    // 构建查询
    let mut query = product::Entity::find();

    // 添加分类筛选
    if let Some(category_id) = params.category_id {
        let subquery = product_category::Entity::find()
            .filter(product_category::Column::CategoryId.eq(category_id))
            .select_only()
            .column(product_category::Column::ProductId)
            .into_query();

        query = query.filter(product::Column::ProductId.in_subquery(subquery));
    }

    // 添加地区筛选
    if let Some(region_id) = params.region_id {
        let subquery = product_region::Entity::find()
            .filter(product_region::Column::RegionId.eq(region_id))
            .select_only()
            .column(product_region::Column::ProductId)
            .into_query();

        query = query.filter(product::Column::ProductId.in_subquery(subquery));
    }

    // 添加搜索条件
    if let Some(search) = &params.search {
        if !search.trim().is_empty() {
            query = query.filter(
                Condition::any()
                    .add(product::Column::ProductName.contains(search))
                    .add(product::Column::ProductDescription.contains(search)),
            );
        }
    }

    // 计算总数
    let total = query.clone().count(db).await?;

    // 分页
    let page = params.page.unwrap_or(1).max(1);
    let page_size = params.limit.unwrap_or(20).min(100); // 限制最大页面大小
    let offset = (page - 1) * page_size;

    // 执行查询
    let products: Vec<product::Model> = query
        .offset(offset as u64)
        .limit(page_size as u64)
        .all(db)
        .await?;

    // 转换为响应格式
    let product_responses: Vec<ProductResponse> = products
        .into_iter()
        .map(|product| ProductResponse {
            product_id: product.product_id,
            product_name: product.product_name,
            product_description: product.product_description,
            product_image: product.product_image,
            product_price: product.product_price.to_string().parse().unwrap_or(0.0),
            product_stock: product.product_stock,
            status: match product.status {
                ::entity::product::ProductStatus::Active => ProductStatus::Active,
                ::entity::product::ProductStatus::Inactive => ProductStatus::Inactive,
            },
            categories: vec![], // TODO: 需要单独查询关联的分类
            regions: vec![],    // TODO: 需要单独查询关联的地区
            created_at: product.created_at.to_string(),
            updated_at: product.updated_at.to_string(),
        })
        .collect();

    Ok(ProductListResponse {
        products: product_responses,
        page,
        limit: page_size,
        total: (total + page_size as u64 - 1) / page_size as u64,
    })
}

pub async fn get_product(
    db: &DatabaseConnection,
    product_id: i64,
    _user: Option<&AuthenticatedUser>,
) -> Result<ProductResponse> {
    // 查找产品
    let product = product::Entity::find_by_id(product_id)
        .one(db)
        .await?
        .ok_or_else(|| anyhow::anyhow!("Product not found"))?;

    // 转换为响应格式
    let product_response = ProductResponse {
        product_id: product.product_id,
        product_name: product.product_name,
        product_description: product.product_description,
        product_image: product.product_image,
        product_price: product.product_price.to_string().parse().unwrap_or(0.0),
        product_stock: product.product_stock,
        status: match product.status {
            ::entity::product::ProductStatus::Active => ProductStatus::Active,
            ::entity::product::ProductStatus::Inactive => ProductStatus::Inactive,
        },
        categories: vec![], // TODO: 需要单独查询关联的分类
        regions: vec![],    // TODO: 需要单独查询关联的地区
        created_at: product.created_at.to_string(),
        updated_at: product.updated_at.to_string(),
    };

    Ok(product_response)
}

pub async fn create_product(
    db: &DatabaseConnection,
    request: CreateProductRequest,
    _user: AuthenticatedUser,
) -> Result<i64> {
    let exsiting_product = product::Entity::find()
        .filter(product::Column::ProductName.eq(&request.product_name))
        .one(db)
        .await
        .map_err(|e| anyhow!("Database error: {}", e))?;

    if exsiting_product.is_some() {
        return Err(anyhow!("Product already exists"));
    }

    let product = product::ActiveModel {
        product_name: Set(request.product_name),
        product_stock: Set(request.product_stock),
        product_description: Set(request.product_description),
        product_price: Set(request.product_price),
        product_image: Set(request.product_image),
        created_at: Set(Utc::now()),
        ..Default::default()
    };

    let product = product
        .insert(db)
        .await
        .map_err(|e| anyhow!("Insert product failed: {:?}", e))?;

    Ok(product.product_id)
}

pub async fn update_product(
    _db: &DatabaseConnection,
    _product_id: i64,
    _request: UpdateProductRequest,
) -> Result<()> {
    Err(anyhow::anyhow!(
        "Product service not available until migration complete"
    ))
}

pub async fn delete_product(_db: &DatabaseConnection, _product_id: i64) -> Result<()> {
    Err(anyhow::anyhow!(
        "Product service not available until migration complete"
    ))
}

pub async fn list_categories(_db: &DatabaseConnection) -> Result<CategoryListResponse> {
    Err(anyhow::anyhow!(
        "Product service not available until migration complete"
    ))
}

pub async fn create_category(
    _db: &DatabaseConnection,
    _request: CreateCategoryRequest,
) -> Result<i64> {
    Err(anyhow::anyhow!(
        "Product service not available until migration complete"
    ))
}

pub async fn list_regions(_db: &DatabaseConnection) -> Result<RegionListResponse> {
    Err(anyhow::anyhow!(
        "Product service not available until migration complete"
    ))
}

pub async fn create_region(_db: &DatabaseConnection, _request: CreateRegionRequest) -> Result<i64> {
    Err(anyhow::anyhow!(
        "Product service not available until migration complete"
    ))
}
