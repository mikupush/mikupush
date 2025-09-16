/// Copyright 2025 Miku Push! Team
///
/// Licensed under the Apache License, Version 2.0 (the "License");
/// you may not use this file except in compliance with the License.
/// You may obtain a copy of the License at
///
///     http://www.apache.org/licenses/LICENSE-2.0
///
/// Unless required by applicable law or agreed to in writing, software
/// distributed under the License is distributed on an "AS IS" BASIS,
/// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
/// See the License for the specific language governing permissions and
/// limitations under the License.
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
