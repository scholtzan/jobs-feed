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
                    .col(ColumnDef::new(Source::Unreachable).boolean())
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

#[derive(DeriveIden)]
pub enum Source {
    Table,
    Id,
    Name,
    Url,
    CreatedAt,
    Selector,
    Pagination,
    Content,
    Favicon,
    Unreachable
}
