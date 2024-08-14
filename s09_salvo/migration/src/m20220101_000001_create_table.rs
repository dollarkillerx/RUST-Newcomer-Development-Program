use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts
        manager
            .create_table(
                Table::create()
                    .table(TimeSeriesPosition::Table)  // table name
                    .if_not_exists()     // create table if not exists
                    .col(
                        ColumnDef::new(TimeSeriesPosition::Id)
                            .string_len(255).not_null().primary_key(),
                    )
                    .col(timestamp_with_time_zone(TimeSeriesPosition::CreatedAt))
                    .col(timestamp_with_time_zone(TimeSeriesPosition::UpdatedAt))
                    .col(timestamp_with_time_zone_null(TimeSeriesPosition::DeletedAt))
                    .col(string_len(TimeSeriesPosition::ClientId,255))
                    .col(big_integer(TimeSeriesPosition::Account))
                    .col(big_integer(TimeSeriesPosition::Leverage))
                    .col(string_len(TimeSeriesPosition::Server,255))
                    .col(string_len(TimeSeriesPosition::Company,255))
                    .col(decimal_len(TimeSeriesPosition::Balance,20,8))
                    .col(decimal_len(TimeSeriesPosition::Profit,20,8))
                    .col(decimal_len(TimeSeriesPosition::Margin,20,8))
                    .col(text(TimeSeriesPosition::Payload))
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts

        manager
            .drop_table(Table::drop().table(TimeSeriesPosition::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum TimeSeriesPosition {
    Table,
    Id,
    CreatedAt,
    UpdatedAt,
    DeletedAt,
    ClientId,
    Account,
    Leverage,
    Server,
    Company,
    Balance,
    Profit,
    Margin,
    Payload
}
