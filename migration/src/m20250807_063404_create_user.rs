use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(User::Table)
                    .if_not_exists()
                    .col(pk_auto(User::Id))
                    .col(uuid(User::Pid))
                    .col(string_uniq(User::Email))
                    .col(string(User::Password))
                    .col(string(User::ApiKey).unique_key())
                    .col(string(User::Name))
                    .col(string_null(User::ResetToken))
                    .col(timestamp_with_time_zone_null(User::ResetSentAt))
                    .col(string_null(User::EmailVerificationToken))
                    .col(timestamp_with_time_zone_null(User::EmailVerificationSentAt))
                    .col(timestamp_with_time_zone_null(User::EmailVerifiedAt))
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(User::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum User {
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
