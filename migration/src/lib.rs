pub use sea_orm_migration::prelude::*;

mod m20251104_161216_create_users;
mod m20251104_162418_create_products;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20251104_161216_create_users::Migration),
            Box::new(m20251104_162418_create_products::Migration),
        ]
    }
}
