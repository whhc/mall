use sea_orm_migration::prelude::extension::postgres::Type;
use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // 创建产品状态枚举类型
        manager
            .create_type(
                Type::create()
                    .as_enum(ProductStatus::Table)
                    .values([ProductStatus::Active, ProductStatus::Inactive])
                    .to_owned(),
            )
            .await?;

        // 创建分类表
        manager
            .create_table(
                Table::create()
                    .table(Category::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Category::CategoryId)
                            .big_integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Category::CategoryName).string().not_null())
                    .col(ColumnDef::new(Category::ParentCategoryId).big_integer())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_category_parent")
                            .from(Category::Table, Category::ParentCategoryId)
                            .to(Category::Table, Category::CategoryId)
                            .on_delete(ForeignKeyAction::SetNull),
                    )
                    .to_owned(),
            )
            .await?;

        // 创建地区表
        manager
            .create_table(
                Table::create()
                    .table(Region::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Region::RegionId)
                            .big_integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Region::RegionName).string().not_null())
                    .to_owned(),
            )
            .await?;

        // 创建产品表
        manager
            .create_table(
                Table::create()
                    .table(Product::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Product::ProductId)
                            .big_integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(Product::ProductName)
                            .string()
                            .not_null()
                            .unique_key(),
                    )
                    .col(ColumnDef::new(Product::ProductDescription).text())
                    .col(
                        ColumnDef::new(Product::ProductPrice)
                            .decimal_len(10, 2)
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(Product::ProductStock)
                            .integer()
                            .not_null()
                            .default(0),
                    )
                    .col(
                        ColumnDef::new(Product::Status)
                            .custom(ProductStatus::Table)
                            .not_null()
                            .default("active"),
                    )
                    .col(ColumnDef::new(Product::ProductImage).string())
                    .col(
                        ColumnDef::new(Product::CreatedAt)
                            .timestamp_with_time_zone()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .col(
                        ColumnDef::new(Product::UpdatedAt)
                            .timestamp_with_time_zone()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .to_owned(),
            )
            .await?;

        // 创建产品分类关联表
        manager
            .create_table(
                Table::create()
                    .table(ProductCategory::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(ProductCategory::ProductId)
                            .big_integer()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(ProductCategory::CategoryId)
                            .big_integer()
                            .not_null(),
                    )
                    .primary_key(
                        Index::create()
                            .col(ProductCategory::ProductId)
                            .col(ProductCategory::CategoryId),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_product_category_product")
                            .from(ProductCategory::Table, ProductCategory::ProductId)
                            .to(Product::Table, Product::ProductId)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_product_category_category")
                            .from(ProductCategory::Table, ProductCategory::CategoryId)
                            .to(Category::Table, Category::CategoryId)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;

        // 创建产品地区关联表
        manager
            .create_table(
                Table::create()
                    .table(ProductRegion::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(ProductRegion::ProductId)
                            .big_integer()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(ProductRegion::RegionId)
                            .big_integer()
                            .not_null(),
                    )
                    .primary_key(
                        Index::create()
                            .col(ProductRegion::ProductId)
                            .col(ProductRegion::RegionId),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_product_region_product")
                            .from(ProductRegion::Table, ProductRegion::ProductId)
                            .to(Product::Table, Product::ProductId)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_product_region_region")
                            .from(ProductRegion::Table, ProductRegion::RegionId)
                            .to(Region::Table, Region::RegionId)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // 删除表的顺序需要考虑外键约束
        manager
            .drop_table(Table::drop().table(ProductRegion::Table).to_owned())
            .await?;

        manager
            .drop_table(Table::drop().table(ProductCategory::Table).to_owned())
            .await?;

        manager
            .drop_table(Table::drop().table(Product::Table).to_owned())
            .await?;

        manager
            .drop_table(Table::drop().table(Region::Table).to_owned())
            .await?;

        manager
            .drop_table(Table::drop().table(Category::Table).to_owned())
            .await?;

        // 删除枚举类型
        manager
            .drop_type(Type::drop().name(ProductStatus::Table).to_owned())
            .await?;

        Ok(())
    }
}

#[derive(DeriveIden)]
enum ProductStatus {
    Table,
    Active,
    Inactive,
}

#[derive(DeriveIden)]
enum Product {
    Table,
    ProductId,
    ProductName,
    ProductDescription,
    ProductPrice,
    ProductStock,
    Status,
    ProductImage,
    CreatedAt,
    UpdatedAt,
}

#[derive(DeriveIden)]
enum Category {
    Table,
    CategoryId,
    CategoryName,
    ParentCategoryId,
}

#[derive(DeriveIden)]
enum Region {
    Table,
    RegionId,
    RegionName,
}

#[derive(DeriveIden)]
enum ProductCategory {
    Table,
    ProductId,
    CategoryId,
}

#[derive(DeriveIden)]
enum ProductRegion {
    Table,
    ProductId,
    RegionId,
}
