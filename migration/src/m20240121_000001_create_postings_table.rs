use sea_orm_migration::prelude::*;

use super::m20240121_000001_create_sources_table::Source;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Posting::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Posting::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Posting::Title).string().not_null())
                    .col(ColumnDef::new(Posting::Description).string().not_null())
                    .col(ColumnDef::new(Posting::Url).string())
                    .col(ColumnDef::new(Posting::CreatedAt).timestamp_with_time_zone().not_null().default(Expr::current_timestamp()))
                    .col(ColumnDef::new(Posting::Seen).boolean().not_null().default(false))
                    .col(ColumnDef::new(Posting::SourceId).integer().not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-posting-source_id")
                            .from(Posting::Table, Posting::SourceId)
                            .to(Source::Table, Source::Id),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Posting::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum Posting {
    Table,
    Id,
    Title,
    Description,
    Url,
    CreatedAt,
    Seen,
    SourceId
}
