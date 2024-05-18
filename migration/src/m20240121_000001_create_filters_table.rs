use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Filter::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Filter::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Filter::Name).string().not_null())
                    .col(ColumnDef::new(Filter::Value).string().not_null())
                    .col(ColumnDef::new(Filter::CreatedAt).timestamp_with_time_zone().default(Expr::current_timestamp()))
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Filter::Table).to_owned())
            .await
    }
}

/// Filter to determine matching job postings from source.
#[derive(DeriveIden)]
enum Filter {
    /// Table
    Table,

    /// Unique identifier
    Id,

    /// Custom name for filter
    Name,

    /// Custom fiter value
    Value,

    /// Timestamp when filter was created
    CreatedAt
}
