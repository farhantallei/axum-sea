use sea_orm_migration::{prelude::*, schema::*};

use crate::m20251104_161216_create_users::Users;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Products::Table)
                    .if_not_exists()
                    .col(integer(Products::Id).auto_increment().primary_key())
                    .col(integer(Products::OwnerId).not_null())
                    .col(text(Products::Title))
                    .col(text_null(Products::Content))
                    .col(decimal(Products::Price).decimal_len(10, 2))
                    .col(timestamp(Products::CreatedAt).default(Keyword::CurrentTimestamp))
                    .col(timestamp(Products::UpdatedAt).default(Keyword::CurrentTimestamp))
                    .to_owned(),
            )
            .await?;

        manager
            .create_foreign_key(
                ForeignKey::create()
                    .from(Products::Table, Products::OwnerId)
                    .to(Users::Table, Users::Id)
                    .on_delete(ForeignKeyAction::Cascade)
                    .on_update(ForeignKeyAction::Cascade)
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Products::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum Products {
    Table,
    Id,
    OwnerId,
    Title,
    Content,
    Price,
    CreatedAt,
    UpdatedAt,
}
