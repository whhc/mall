use ::entity::{category, product, product_category, product_region, region};
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
            status: product.status,
            categories: vec![], // 在列表视图中暂时为空，避免N+1查询问题
            regions: vec![],    // 在列表视图中暂时为空，避免N+1查询问题
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
        .ok_or_else(|| anyhow::anyhow!("Get product Error: product not found"))?;

    let categroy_ids = product_category::Entity::find()
        .filter(product_category::Column::ProductId.eq(product_id))
        .all(db)
        .await
        .map_err(|e| anyhow!("Product {product_id} get category ids error: {e}"))?
        .into_iter()
        .map(|pc| pc.category_id)
        .collect::<Vec<_>>();

    let categories = category::Entity::find()
        .filter(category::Column::CategoryId.is_in(categroy_ids))
        .all(db)
        .await
        .map_err(|e| anyhow!("Product {product_id} get categories error: {e}"))?
        .into_iter()
        .map(|category| CategoryInfo {
            category_id: category.category_id,
            category_name: category.category_name,
            parent_category_id: category.parent_category_id,
        })
        .collect();

    // 查询关联的地区
    let region_ids = product_region::Entity::find()
        .filter(product_region::Column::ProductId.eq(product_id))
        .all(db)
        .await
        .map_err(|e| anyhow!("Product {product_id} get region ids error: {e}"))?
        .into_iter()
        .map(|pr| pr.region_id)
        .collect::<Vec<_>>();

    let regions = region::Entity::find()
        .filter(region::Column::RegionId.is_in(region_ids))
        .all(db)
        .await
        .map_err(|e| anyhow!("Product {product_id} get regions error: {e}"))?
        .into_iter()
        .map(|region| RegionInfo {
            region_id: region.region_id,
            region_name: region.region_name,
        })
        .collect();

    // 转换为响应格式
    let product_response = ProductResponse {
        product_id: product.product_id,
        product_name: product.product_name,
        product_description: product.product_description,
        product_image: product.product_image,
        product_price: product.product_price.to_string().parse().unwrap_or(0.0),
        product_stock: product.product_stock,
        status: product.status,
        categories,
        regions,
        created_at: product.created_at.to_string(),
        updated_at: product.updated_at.to_string(),
    };

    Ok(product_response)
}

pub async fn create_product(db: &DatabaseConnection, request: CreateProductRequest) -> Result<i64> {
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
        status: Set(request.status),
        ..Default::default()
    };

    let product = product
        .insert(db)
        .await
        .map_err(|e| anyhow!("Insert product failed: {:?}", e))?;

    let product_id = product.product_id;
    let categories = request.category_ids;
    for category_id in categories {
        let product_category = product_category::ActiveModel {
            product_id: Set(product_id),
            category_id: Set(category_id),
            ..Default::default()
        };

        product_category
            .insert(db)
            .await
            .map_err(|e| anyhow!("Insert product_category failed: {:?}", e))?;
    }
    let regions = request.region_ids;
    for region_id in regions {
        let product_region = product_region::ActiveModel {
            product_id: Set(product_id),
            region_id: Set(region_id),
            ..Default::default()
        };

        product_region
            .insert(db)
            .await
            .map_err(|e| anyhow!("Insert product_region failed: {:?}", e))?;
    }

    Ok(product.product_id)
}

pub async fn update_product(
    db: &DatabaseConnection,
    product_id: i64,
    request: UpdateProductRequest,
) -> Result<()> {
    let product = product::Entity::find_by_id(product_id)
        .one(db)
        .await
        .map_err(|e| anyhow!("Get product database error: {}", e))?
        .ok_or_else(|| anyhow!("Product not found"))?;

    let mut product = product.into_active_model();

    if let Some(product_name) = request.product_name {
        product.product_name = Set(product_name);
    }
    if let Some(product_description) = request.product_description {
        product.product_description = Set(Some(product_description));
    }

    if let Some(product_image) = request.product_image {
        product.product_image = Set(Some(product_image));
    }

    if let Some(product_price) = request.product_price {
        product.product_price = Set(product_price);
    }
    if let Some(product_stock) = request.product_stock {
        product.product_stock = Set(product_stock);
    }
    if let Some(status) = request.status {
        product.status = Set(status);
    }

    product.updated_at = Set(Utc::now());

    if let Some(category_ids) = request.category_ids {
        product_category::Entity::delete_many()
            .filter(product_category::Column::ProductId.eq(product_id))
            .exec(db)
            .await
            .map_err(|e| anyhow!("Delete product_category database error: {}", e))?;
        for category_id in category_ids {
            let product_category = product_category::ActiveModel {
                product_id: Set(product_id),
                category_id: Set(category_id),
                ..Default::default()
            };

            product_category
                .insert(db)
                .await
                .map_err(|e| anyhow!("Insert product_category failed: {:?}", e))?;
        }
    }

    if let Some(region_ids) = request.region_ids {
        product_region::Entity::delete_many()
            .filter(product_region::Column::ProductId.eq(product_id))
            .exec(db)
            .await
            .map_err(|e| anyhow!("Delete product_region database error: {}", e))?;
        for region_id in region_ids {
            let product_region = product_region::ActiveModel {
                product_id: Set(product_id),
                region_id: Set(region_id),
                ..Default::default()
            };

            product_region
                .insert(db)
                .await
                .map_err(|e| anyhow!("Insert product_region failed: {:?}", e))?;
        }
    }

    product
        .update(db)
        .await
        .map_err(|e| anyhow!("Update product database error: {}", e))?;

    Ok(())
}

pub async fn delete_product(db: &DatabaseConnection, product_id: i64) -> Result<()> {
    product::Entity::delete_by_id(product_id)
        .exec(db)
        .await
        .map_err(|e| anyhow!("Delete product database error: {}", e))?;
    product_category::Entity::delete_many()
        .filter(product_category::Column::ProductId.eq(product_id))
        .exec(db)
        .await
        .map_err(|e| anyhow!("Delete product_category database error: {}", e))?;
    product_region::Entity::delete_many()
        .filter(product_region::Column::ProductId.eq(product_id))
        .exec(db)
        .await
        .map_err(|e| anyhow!("Delete product_region database error: {}", e))?;
    Ok(())
}

pub async fn list_categories(db: &DatabaseConnection) -> Result<CategoryListResponse> {
    let categories = category::Entity::find()
        .all(db)
        .await
        .map_err(|e| anyhow!("Get cateories database error: {}", e))?;

    let category_responses: Vec<CategoryInfo> = categories
        .into_iter()
        .map(|category| CategoryInfo {
            category_id: category.category_id,
            category_name: category.category_name,
            parent_category_id: category.parent_category_id,
        })
        .collect();

    Ok(CategoryListResponse {
        categories: category_responses,
    })
}

pub async fn create_category(
    db: &DatabaseConnection,
    request: CreateCategoryRequest,
) -> Result<i64> {
    let exsiting_category = category::Entity::find()
        .filter(category::Column::CategoryName.eq(&request.category_name))
        .one(db)
        .await
        .map_err(|e| anyhow!("Database error: {}", e))?;

    if exsiting_category.is_some() {
        return Err(anyhow!("Category already exists"));
    }

    let category = category::ActiveModel {
        category_name: Set(request.category_name),
        ..Default::default()
    };

    let category = category
        .insert(db)
        .await
        .map_err(|e| anyhow!("Insert category failed: {:?}", e))?;

    Ok(category.category_id)
}

pub async fn list_regions(db: &DatabaseConnection) -> Result<RegionListResponse> {
    let regions = region::Entity::find()
        .all(db)
        .await
        .map_err(|e| anyhow!("Get regions database error: {}", e))?;

    let region_responses: Vec<RegionInfo> = regions
        .into_iter()
        .map(|region| RegionInfo {
            region_id: region.region_id,
            region_name: region.region_name,
        })
        .collect();

    Ok(RegionListResponse {
        regions: region_responses,
    })
}

pub async fn create_region(db: &DatabaseConnection, request: CreateRegionRequest) -> Result<i64> {
    let existing_region = region::Entity::find()
        .filter(region::Column::RegionName.eq(&request.region_name))
        .one(db)
        .await
        .map_err(|e| anyhow!("Database error: {}", e))?;

    if existing_region.is_some() {
        return Err(anyhow!("Region already exists"));
    }

    let region = region::ActiveModel {
        region_name: Set(request.region_name),
        ..Default::default()
    };

    let region = region
        .insert(db)
        .await
        .map_err(|e| anyhow!("Insert region failed: {:?}", e))?;

    Ok(region.region_id)
}
