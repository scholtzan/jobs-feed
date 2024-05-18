use sea_orm_migration::prelude::*;

use super::m20240121_000001_create_postings_table::Posting;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Embedding::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Embedding::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Embedding::Vector).array(ColumnType::Float))
                    .col(ColumnDef::new(Embedding::PostingId).integer())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-embedding-posting_id")
                            .from(Embedding::Table, Embedding::PostingId)
                            .to(Posting::Table, Posting::Id),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Embedding::Table).to_owned())
            .await
    }
}

/// Representation of posting as LLM embedding.
/// Used to determine similar postings.
#[derive(DeriveIden)]
pub enum Embedding {
    /// Table
    Table,

    /// Unique identifier for embedding
    Id,

    /// Embedding vector
    Vector,

    /// ID of posting represented by embedding
    PostingId,
}
