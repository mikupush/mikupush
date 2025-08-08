use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Upload::Table)
                    .if_not_exists()
                    .col(pk_uuid(Upload::Id))
                    .col(string(Upload::Name))
                    .col(big_integer(Upload::Size))
                    .col(string(Upload::MimeType))
                    .col(text(Upload::Path))
                    .col(text_null(Upload::Url))
                    .col(date_time(Upload::CreatedAt))
                    .col(string(Upload::Status))
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Upload::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum Upload {
    Table,
    Id,
    Name,
    Size,
    MimeType,
    Path,
    Url,
    CreatedAt,
    Status
}
