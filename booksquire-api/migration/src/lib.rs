pub use sea_orm_migration::prelude::*;

// Add each migration file as a module
mod m20221106_152901_2022_11_06_1520_example;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            // Define the order of migrations.
            Box::new(m20221106_152901_2022_11_06_1520_example::Migration),
        ]
    }
}
