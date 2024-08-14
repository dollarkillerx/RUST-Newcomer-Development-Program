
``` 
sea-orm-cli migrate init  // 初始化
sea-orm-cli migrate up    // 迁移
sea-orm-cli migrate down  // drop table

defalut read .env DATABASE_URL 

manager
    .create_table(
        Table::create()
            .table(Post::Table)  // table name
            .if_not_exists()     // create table if not exists
            .col(pk_auto(Post::Id))  // primary key column auto increment
            .col(string(Post::Title)) // column title string
            .col(string(Post::Text))  // column text string
            .to_owned(),
    )
    .await
    
    
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
    

manager
    .create_table(
        Table::create()
            .table(TimeSeriesPosition::Table)  // table name
            .if_not_exists()     // create table if not exists
            .col(
                ColumnDef::new(TimeSeriesPosition::Id)
                    .string_len(255).not_null().primary_key(),
            )
            .col(ColumnDef::new(Account::CreatedAt).timestamp_with_time_zone())
            .col(ColumnDef::new(Account::UpdatedAt).timestamp_with_time_zone())
            .col(ColumnDef::new(Account::DeletedAt).timestamp_with_time_zone().null())
            .col(ColumnDef::new(Account::ClientId).string_len(255).not_null())
            .col(ColumnDef::new(Account::Account).big_integer().not_null())
            .col(ColumnDef::new(Account::Leverage).integer().not_null())
            .col(ColumnDef::new(Account::Server).string_len(255).not_null())
            .col(ColumnDef::new(Account::Company).string_len(255).not_null())
            .col(ColumnDef::new(Account::Balance).decimal_len(20, 8).not_null())
            .col(ColumnDef::new(Account::Profit).decimal_len(20, 8).not_null())
            .col(ColumnDef::new(Account::Margin).decimal_len(20, 8).not_null())
            .to_owned(),
    )
    .await
    
cargo new entity
sea-orm-cli generate entity  -o src/entity --with-serde both
```