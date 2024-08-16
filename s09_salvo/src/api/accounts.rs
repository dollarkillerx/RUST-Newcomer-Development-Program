use chrono::{Datelike, NaiveDate, Utc};
use salvo::prelude::*;
use sea_orm::{ColumnTrait, DatabaseConnection, DbErr, EntityTrait, QuerySelect};
use sea_orm::prelude::Decimal;
use serde::Serialize;
use crate::app_state::AppState;
use crate::entity::{positions, time_series_position};
use crate::errors::CustomError;
use crate::models::req::RespResult;
use sea_orm::QueryFilter;

#[handler]
pub async fn accounts(req: &mut Request, res: &mut Response, depot: &mut Depot) -> Result<(), CustomError> {
    let state = depot.obtain::<AppState>().unwrap();
    let accts = state.storage.get_accounts().await?;

    res.render(Json(RespResult::new(
        200,
        "ok".to_owned(),
        accts,
    )));
    Ok(())
}

#[derive(Serialize)]
struct ProfitResult {
    period: String,
    max_profit: f64,
    min_profit: f64,
}

#[derive(Serialize)]
struct AccountResponse {
    positions: Vec<positions::Model>,
    profits: Vec<ProfitResult>,
}

async fn get_daily_profit(
    db: &DatabaseConnection,
    client_id: &str,
    date: NaiveDate,
) -> Result<ProfitResult, DbErr> {
    // 一天的开始时间和结束时间
    let start_of_day = date.and_hms(0, 0, 0);
    let end_of_day = date.and_hms(23, 59, 59);

    let result = time_series_position::Entity::find()
        .filter(time_series_position::Column::ClientId.eq(client_id))
        .filter(time_series_position::Column::CreatedAt.between(start_of_day, end_of_day))
        .select_only()
        .column_as(time_series_position::Column::Profit.max(), "max_profit")
        .column_as(time_series_position::Column::Profit.min(), "min_profit")
        .into_tuple::<(Option<Decimal>, Option<Decimal>)>()
        .one(db)
        .await?;

    let (max_profit, min_profit) = result.unwrap_or((None, None));

    Ok(ProfitResult {
        period: date.to_string(),
        max_profit: max_profit.unwrap_or(Decimal::ZERO).to_string().parse().unwrap_or(0.0),
        min_profit: min_profit.unwrap_or(Decimal::ZERO).to_string().parse().unwrap_or(0.0),
    })
}

#[handler]
pub async fn account(req: &mut Request, res: &mut Response, depot: &mut Depot) -> Result<(), CustomError> {
    let account_param = req.param::<String>("account").ok_or(CustomError::ParamError("error".into()))?;

    let state = depot.obtain::<AppState>().unwrap();
    let pos = state.storage.get_positions(&account_param).await?;

    // 今日日期
    let today = Utc::now().date_naive();

    // 最近5天的日期
    let now = Utc::now().naive_utc();
    let first_of_month = NaiveDate::from_ymd_opt(now.year(), now.month(), 1).unwrap();

    let mut profit_data = Vec::new();

    // Helper function to get profit data
    async fn get_profit(state: &AppState, client_id: &str, date_filter: Option<NaiveDate>) -> Result<ProfitResult, DbErr> {
        let mut query = time_series_position::Entity::find()
            .filter(time_series_position::Column::ClientId.eq(client_id));

        if let Some(date) = date_filter {
            query = query.filter(time_series_position::Column::CreatedAt.gte(date.and_hms(0, 0, 0)));
        }

        let result = query
            .select_only()
            .column_as(time_series_position::Column::Profit.max(), "max_profit")
            .column_as(time_series_position::Column::Profit.min(), "min_profit")
            .into_tuple::<(Option<Decimal>, Option<Decimal>)>()
            .one(&state.storage.db)
            .await?;


        let (max_profit, min_profit) = result.unwrap_or((None, None));

        Ok(ProfitResult {
            period: date_filter.map_or("all_time".to_string(), |d| d.to_string()),
            max_profit: max_profit.unwrap_or(Decimal::ZERO).to_string().parse().unwrap_or(0.0),
            min_profit: min_profit.unwrap_or(Decimal::ZERO).to_string().parse().unwrap_or(0.0),
        })
    }

    // 当天最高利润和最低利润
    let today_profit = get_profit(state, &account_param, Some(today)).await?;
    profit_data.push(today_profit);

    // 获取最近5天的利润数据
    for i in 0..5 {
        let date = Utc::now().date_naive() - chrono::Duration::days(i);
        let daily_profit = get_daily_profit(&state.storage.db, &account_param, date).await?;
        profit_data.push(daily_profit);
    }
    // 本月最高利润和最低利润
    let monthly_profit = get_profit(state, &account_param, Some(first_of_month)).await?;
    profit_data.push(monthly_profit);

    // 历史最高利润和最低利润
    let all_time_profit = get_profit(state, &account_param, None).await?;
    profit_data.push(all_time_profit);

    // 返回利润统计数据
    res.render(Json(RespResult::new(
        200,
        "ok".to_owned(),
        AccountResponse {
            positions: pos,
            profits: profit_data,
        },
    )));

    Ok(())
}

#[handler]
pub async fn account_charts(req: &mut Request, res: &mut Response, depot: &mut Depot) -> Result<(), CustomError> {
    let account_param = req.param::<String>("account").ok_or(CustomError::ParamError("error".into()))?;
    let state = depot.obtain::<AppState>().unwrap();

    let positions = state.storage.get_time_series_positions(&account_param).await?;

    res.render(Json(RespResult::new(
        200,
        "ok".to_owned(),
        positions,
    )));
    Ok(())
}
