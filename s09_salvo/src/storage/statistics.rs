use chrono::Utc;
use futures::{stream, StreamExt};
use redis::AsyncCommands;
use sea_orm::ActiveValue::Set;
use sea_orm::prelude::Decimal;
use serde_json::json;
use crate::entity::{account, positions, time_series_position};
use crate::errors::CustomError;
use crate::models::req::{Positions};
use crate::storage::storage::Storage;
use sea_orm::entity::prelude::*;
use sea_orm::TryIntoModel;

impl Storage {
    pub async fn statistics(&self, account: account::Model, positions: Vec<Positions>) -> Result<(), CustomError> {
        if positions.len() == 0 {
            return Ok(());
        }

        // 1. 获取够着model Positions
        let pos: Vec<positions::Model> = stream::iter(&positions)
            .filter_map(|x| async {
                // 时间使用系统时间
                let opening_time_system = match self.gen_time(format!("{}_{}_{}", &account.client_id, "openingTimeSystem", x.order_id).as_str()).await {
                    Ok(time) => Some(time),
                    Err(_) => None,
                };

                // 如果获取失败抛弃这条数据
                if opening_time_system == None {
                    return None;
                }

                Some(positions::Model {
                    id: xid::new().to_string(),
                    created_at: None,
                    updated_at: None,
                    deleted_at: None,
                    client_id: account.client_id.clone(),
                    order_id: x.order_id,
                    direction: x.direction.to_string(),
                    symbol: x.symbol.clone(),
                    magic: x.magic,
                    open_price: Decimal::from_f64_retain(x.open_price).unwrap(),
                    volume: Decimal::from_f64_retain(x.volume).unwrap(),
                    market: Decimal::from_f64_retain(x.market).unwrap(),
                    swap: Decimal::from_f64_retain(x.swap).unwrap(),
                    profit: Decimal::from_f64_retain(x.profit).unwrap(),
                    common: x.common.clone(),
                    opening_time: x.opening_time,
                    closing_time: x.closing_time,
                    common_internal: Option::from(x.common_internal.clone()),
                    opening_time_system,
                    closing_time_system: x.closing_time_system,
                })
            })
            .collect()
            .await;

        let mut before = "".to_owned();
        let mut after = "".to_owned();
        pos.iter().for_each(|x| {
            after.push_str(format!("{}", x.order_id).as_str());
        });

        // 获取上一个数据
        let last_time_series_position = self.get_last_time_series_position(&account.client_id).await;
        if last_time_series_position == None {
            // 使用当前数据进行更新
            let active_model = time_series_position::ActiveModel {
                id: Set(xid::new().to_string()),
                created_at: Set(DateTimeWithTimeZone::from(Utc::now())), // 使用带时区的 `DateTime<Utc>`
                updated_at: Set(DateTimeWithTimeZone::from(Utc::now())), // 使用带时区的 `DateTime<Utc>`
                deleted_at: Set(None),
                client_id: Set(account.client_id.clone()),
                account: Set(account.account),
                leverage: Set(account.leverage as i64),
                server: Set("".to_owned()),
                company: Set("".to_owned()),
                balance: Set(account.balance.clone()),
                profit: Set(account.profit.clone()),
                margin: Set(account.margin.clone()),
                payload: Set(json!(&positions).to_string()),
            };

            // time_series_position::Entity::insert(active_model.clone()).exec(&self.db).await.map_err(|err| {
            //     // 将 sea_orm 的 DbErr 转换为你的自定义错误类型
            //     CustomError::ParamError(format!("Database error: {}", err))
            // })?;

            time_series_position::Entity::insert(active_model.clone()).exec(&self.db).await?;

            let model: time_series_position::Model = active_model.try_into_model()?;

            // 插入redis
            self.set_time_series_position(&account.client_id, &model).await?;
            return Ok(());
        }

        let last_time_series_position = last_time_series_position.unwrap();

        let pos2: Vec<positions::Model> = serde_json::from_str(last_time_series_position.payload.as_str())?;
        pos2.iter().for_each(|x| {
            before.push_str(format!("{}", x.order_id).as_str());
        });

        if after.eq(before.as_str()) {
            if (account.profit - last_time_series_position.profit).abs() < Decimal::from(2) {
                return Ok(());
            }
        }

        let active_model = time_series_position::ActiveModel {
            id: Set(xid::new().to_string()),
            created_at: Set(DateTimeWithTimeZone::from(Utc::now())), // 使用带时区的 `DateTime<Utc>`
            updated_at: Set(DateTimeWithTimeZone::from(Utc::now())), // 使用带时区的 `DateTime<Utc>`
            deleted_at: Set(None),
            client_id: Set(account.client_id.clone()),
            account: Set(account.account),
            leverage: Set(account.leverage as i64),
            server: Set("".to_owned()),
            company: Set("".to_owned()),
            balance: Set(account.balance.clone()),
            profit: Set(account.profit.clone()),
            margin: Set(account.margin.clone()),
            payload: Set(json!(&positions).to_string()),
        };
        time_series_position::Entity::insert(active_model.clone()).exec(&self.db).await?;

        let model: time_series_position::Model = active_model.try_into_model()?;

        // 插入redis
        self.set_time_series_position(&account.client_id, &model).await?;

        Ok(())
    }

    async fn get_last_time_series_position(&self, client_id: &str) -> Option<time_series_position::Model> {
        let mut conn = self.redis_conn.get_multiplexed_async_connection().await.unwrap();
        match conn.get::<_, Option<String>>(format!("{}_{}", client_id, "TimeSeriesPosition")).await {
            Ok(Some(r)) => {
                // 尝试解析 JSON 字符串
                match serde_json::from_str::<time_series_position::Model>(&r) {
                    Ok(model) => Some(model),
                    Err(_) => None, // 如果解析失败，则返回 None
                }
            }
            Ok(None) => None, // 如果 Redis 中没有该键，则返回 None
            Err(_) => None, // 处理 Redis 连接或命令执行错误
        }
    }

    async fn set_time_series_position(&self, client_id: &str, time_series_position: &time_series_position::Model) -> Result<(), CustomError> {
        let mut conn = self.redis_conn.get_multiplexed_async_connection().await.unwrap();
        const ONE_MONTH_IN_SECONDS: u64 = 10 * 24 * 60 * 60;
        conn.set_ex::<_, _, ()>(format!("{}_{}", client_id, "TimeSeriesPosition"), json!(time_series_position).to_string(), ONE_MONTH_IN_SECONDS).await.map_err(|err| {
            // 将 sea_orm 的 DbErr 转换为你的自定义错误类型
            CustomError::ParamError(format!("Database error: {}", err))
        })?;
        Ok(())
    }
}