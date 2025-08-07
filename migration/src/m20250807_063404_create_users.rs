use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Users::Table)
                    .if_not_exists()
                    .col(pk_auto(Users::Id))
                    .col(uuid(Users::Pid))
                    .col(string_uniq(Users::Email))
                    .col(string(Users::Password))
                    .col(string(Users::ApiKey).unique_key())
                    .col(string(Users::Name))
                    .col(string_null(Users::ResetToken))
                    .col(timestamp_null(Users::ResetSentAt))
                    .col(string_null(Users::EmailVerificationToken))
                    .col(timestamp_null(Users::EmailVerificationSentAt))
                    .col(timestamp_null(Users::EmailVerifiedAt))
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Users::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum Users {
    Table,
    Id,
    Pid, // UUID 唯一标识符
    Email,
    Password, // 密码哈希
    ApiKey,   // api 密钥
    Name,
    ResetToken,              // 密码重置令牌
    ResetSentAt,             // 重置令牌发送时间
    EmailVerificationToken,  // 邮箱验证令牌
    EmailVerificationSentAt, // 验证令牌发送时间
    EmailVerifiedAt,         //邮箱验证时间
}
