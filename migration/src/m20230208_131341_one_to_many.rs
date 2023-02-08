use sea_orm_migration::{prelude::*, sea_orm::Schema};
use entity::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let builder = manager.get_database_backend();
        let schema = Schema::new(builder);

        manager
        .create_table(schema.create_table_from_entity(figure::Entity))
        .await

    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {   
        manager
        .drop_table(Table::drop().table(figure::Entity).to_owned())
        .await
    }
}

