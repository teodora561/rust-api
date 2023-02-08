pub use sea_orm_migration::prelude::*;

mod m20220101_000001_create_table;
mod m20230207_162659_one_to_one;
mod m20230208_123127_add_filling_try_2;
mod m20230208_131341_one_to_many;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20220101_000001_create_table::Migration),
            Box::new(m20230207_162659_one_to_one::Migration),
            Box::new(m20230208_123127_add_filling_try_2::Migration),
            Box::new(m20230208_131341_one_to_many::Migration),
        ]
    }
}
