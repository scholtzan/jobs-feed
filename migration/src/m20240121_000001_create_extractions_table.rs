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
                    .table(Extraction::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Extraction::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Extraction::CreatedAt).timestamp_with_time_zone().default(Expr::current_timestamp()))
                    .col(ColumnDef::new(Extraction::Model).string())
                    .col(ColumnDef::new(Extraction::PromptTokens).integer())
                    .col(ColumnDef::new(Extraction::CompletionTokens).integer())
                    .col(ColumnDef::new(Extraction::Cost).float())
                    .col(ColumnDef::new(Extraction::SourceId).integer())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-extraction-source_id")
                            .from(Extraction::Table, Extraction::SourceId)
                            .to(Source::Table, Source::Id),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Extraction::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub enum Extraction {
    Table,
    Id,
    CreatedAt,
    SourceId,
    Model,
    PromptTokens,
    CompletionTokens,
    Cost
}
