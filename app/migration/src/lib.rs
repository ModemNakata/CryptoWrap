pub use sea_orm_migration::prelude::*;

mod m20260220_121747_tokens_table;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![Box::new(m20260220_121747_tokens_table::Migration)]
    }
}
