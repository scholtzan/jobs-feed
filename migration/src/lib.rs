pub use sea_orm_migration::prelude::*;

mod m20240121_000001_create_sources_table;
mod m20240121_000001_create_settings_table;
mod m20240121_000001_create_postings_table;
mod m20240121_000001_create_filters_table;
mod m20240121_000001_create_suggestions_table;
mod m20240121_000001_create_embeddings_table;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20240121_000001_create_settings_table::Migration),
            Box::new(m20240121_000001_create_sources_table::Migration),
            Box::new(m20240121_000001_create_filters_table::Migration),
            Box::new(m20240121_000001_create_postings_table::Migration),
            Box::new(m20240121_000001_create_suggestions_table::Migration),
            Box::new(m20240121_000001_create_embeddings_table::Migration),
        ]
    }
}
