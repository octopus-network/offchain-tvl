pub use sea_orm_migration::prelude::*;

mod m20241210_000001_one;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
	fn migrations() -> Vec<Box<dyn MigrationTrait>> {
		vec![Box::new(m20241210_000001_one::Migration)]
	}
}
