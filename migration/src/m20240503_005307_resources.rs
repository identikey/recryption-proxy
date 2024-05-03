use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                table_auto(Resources::Table)
                    .col(pk_auto(Resources::Id))
                    .col(string_null(Resources::Name))
                    .col(string_null(Resources::Hash))
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Resources::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum Resources {
    Table,
    Id,
    Name,
    Hash,
    
}


