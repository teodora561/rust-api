use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts
        manager
            .alter_table(
                TableAlterStatement::new()
                    .table(Fruit::Table)
                    .add_column(ColumnDef::new(Fruit::CakeId).integer().not_null().unique_key())
                    .to_owned(),
            )
            .await?;
        manager
            .create_foreign_key(
                ForeignKey::create()
                    .name("CakeId")
                    .from(Fruit::Table, Fruit::CakeId)
                    .to(Cake::Table, Cake::Id)
                    .on_delete(ForeignKeyAction::Cascade)
                    .on_update(ForeignKeyAction::Cascade)
                    .to_owned(),
            )
            .await?;
        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts
        manager
            .drop_foreign_key(ForeignKeyDropStatement::new()
                                    .table(Fruit::Table)
                                    .name("CakeId")
                                    .to_owned()
            )
            .await?;

        manager
            .alter_table(
                TableAlterStatement::new()
                    .drop_column(Fruit::CakeId)
                    .table(Fruit::Table)
                    .to_owned(),
            )
            .await?;
            Ok(())
        
    }
}

/// Learn more at https://docs.rs/sea-query#iden
#[derive(Iden)]
enum Cake {
    Table,
    Id,
    Name,
}

#[derive(Iden)]
enum Fruit {
    Table,
    Id,
    Name, 
    CakeId
}