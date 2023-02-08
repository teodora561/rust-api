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
        .create_table(schema.create_table_from_entity(cake::Entity))
        .await?;

        manager
        .create_table(schema.create_table_from_entity(fruit::Entity))
        .await?;

        manager
        .create_table(schema.create_table_from_entity(filling::Entity))
        .await?;
    manager
        .create_table(schema.create_table_from_entity(cake_filling::Entity))
        .await

    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
        .drop_table(Table::drop().table(filling::Entity).to_owned())
        .await?;
    manager
        .drop_table(Table::drop().table(cake_filling::Entity).to_owned())
        .await?;
    manager
        .drop_table(Table::drop().table(cake::Entity).to_owned())
        .await
    }
}

/// Learn more at https://docs.rs/sea-query#iden
#[derive(Iden)]
enum Post {
    Table,
    Id,
    Title,
    Text,
}
