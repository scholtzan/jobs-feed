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
                    .table(Suggestion::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Suggestion::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Suggestion::Name).string().not_null())
                    .col(ColumnDef::new(Suggestion::Url).string().not_null())
                    .col(ColumnDef::new(Suggestion::SourceId).integer())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-suggestion-source_id")
                            .from(Suggestion::Table, Suggestion::SourceId)
                            .to(Source::Table, Source::Id),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Suggestion::Table).to_owned())
            .await
    }
}

/// Suggestions that are similar to a specific source
#[derive(DeriveIden)]
pub enum Suggestion {
    /// Table
    Table,

    /// Unique identifier
    Id,

    /// Name of a source that is similar to an existing one
    Name,

    /// URL to similar source
    Url,

    /// ID to source that was used as reference to find the source suggestion
    SourceId,
}
