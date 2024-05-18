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
                    .col(ColumnDef::new(Posting::Description).string())
                    .col(ColumnDef::new(Posting::Url).string())
                    .col(ColumnDef::new(Posting::CreatedAt).timestamp_with_time_zone().default(Expr::current_timestamp()))
                    .col(ColumnDef::new(Posting::Seen).boolean().default(false))
                    .col(ColumnDef::new(Posting::SourceId).integer())
                    .col(ColumnDef::new(Posting::Bookmarked).boolean().default(false))
                    .col(ColumnDef::new(Posting::Content).string())
                    .col(ColumnDef::new(Posting::IsMatch).boolean())
                    .col(ColumnDef::new(Posting::MatchSimilarity).float())
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

/// Job posting
#[derive(DeriveIden)]
pub enum Posting {
    /// Table
    Table,

    /// Unique identifier
    Id,

    /// Extracted title of job posting
    Title,

    /// Extracted job posting description
    Description,

    /// Url to job posting
    Url,

    /// Timestamp when job posting was extracted and stored
    CreatedAt,

    /// Whether the job posting has been read
    Seen,

    /// ID to source posting was extracted from
    SourceId,

    /// Whether the posting has been bookmarked
    Bookmarked,

    /// Full text content of the job posting page
    Content,

    /// Whether the posting is a good match
    /// true = good match
    /// false = not a good match
    /// null = neutral
    IsMatch,

    /// Similarity score based on previously liked/disliked postings
    MatchSimilarity
}
