use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Source::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Source::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Source::Name).string().not_null())
                    .col(ColumnDef::new(Source::Url).string().not_null())
                    .col(ColumnDef::new(Source::CreatedAt).timestamp_with_time_zone().default(Expr::current_timestamp()))
                    .col(ColumnDef::new(Source::Selector).string())
                    .col(ColumnDef::new(Source::Pagination).string())
                    .col(ColumnDef::new(Source::Content).text())
                    .col(ColumnDef::new(Source::Favicon).string())
                    .col(ColumnDef::new(Source::Unreachable).boolean().default(false))
                    .col(ColumnDef::new(Source::Deleted).boolean().default(false).not_null())
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Source::Table).to_owned())
            .await
    }
}

/// Source to retrieve job postings from
#[derive(DeriveIden)]
pub enum Source {
    /// Table
    Table,

    /// Unique identifier
    Id,

    /// User-defined name for source
    Name,

    /// URL to source
    Url,

    /// Timestamp source was created
    CreatedAt,

    /// CSS selector to extract job postings from
    /// This is to reduce the input size and reduce cost
    Selector,

    /// CSS selector to the pagination link or button
    /// This is so the app can automatically open more source pages to extract postings 
    Pagination,

    /// Cached source content
    /// Only analyse newly added source content for new job postings
    /// This is to reduce cost and increase performance
    Content,

    /// URL to favicon shown for source
    Favicon,

    /// Whether the URL could be successfully opened
    Unreachable,

    /// Whether the source was deleted
    /// Using a flag here so that related postings don't get removed
    Deleted
}
