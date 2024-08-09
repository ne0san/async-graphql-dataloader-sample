use entity::item;
use entity::user;
use sea_orm_migration::prelude::*;
#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                sea_query::Table::create()
                    .table(item::Entity)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(item::Column::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(item::Column::UserId).integer().not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("FK_item_user")
                            .to_tbl(user::Entity)
                            .to_col(user::Column::Id)
                            .from_tbl(item::Entity)
                            .from_col(item::Column::UserId),
                    )
                    .col(ColumnDef::new(item::Column::Name).string().not_null())
                    .col(ColumnDef::new(item::Column::Description).text())
                    .col(
                        ColumnDef::new(item::Column::Deleted)
                            .boolean()
                            .not_null()
                            .default(false),
                    )
                    .col(
                        ColumnDef::new(item::Column::CreatedAt)
                            .timestamp_with_time_zone()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(item::Column::UpdatedAt)
                            .timestamp_with_time_zone()
                            .not_null(),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(sea_query::Table::drop().table(item::Entity).to_owned())
            .await
    }
}
