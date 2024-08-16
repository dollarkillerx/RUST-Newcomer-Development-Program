use sea_orm::prelude::Decimal;
use futures::stream::{self, StreamExt};
use redis::AsyncCommands;
use crate::entity::positions;
use crate::enums::enums::CacheKey;
use crate::errors::CustomError;
use crate::models::req::Positions;
use crate::storage::storage::Storage;

impl Storage {
    pub async fn update_positions(&self, client_id: &str, positions_input: &Vec<Positions>) -> Result<(), CustomError> {
        // 1. 提取需要的数据
        let pos: Vec<positions::Model> = stream::iter(positions_input)
            .filter_map(|x| async {
                // 时间使用系统时间
                let opening_time_system = match self.gen_time(format!("{}_{}_{}", client_id, "openingTimeSystem", x.order_id).as_str()).await {
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
                    client_id: client_id.to_string(),
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
                    closing_time_system: Option::from(x.closing_time_system),
                })
            })
            .collect()
            .await;


        // 存储数据  时间30天
        let cache_key = CacheKey::CachePositions.get_key(&client_id);
        let account_json = serde_json::to_string(&pos)?;
        let mut conn = self.redis_conn.get_multiplexed_async_connection().await?;
        const ONE_MONTH_IN_SECONDS: u64 = 30 * 24 * 60 * 60;
        conn.set_ex(&cache_key, &account_json, ONE_MONTH_IN_SECONDS).await?;

        Ok(())
    }

    pub async fn get_positions(&self, client_id: &str) -> Result<Vec<positions::Model>, CustomError> {
        let cache_key = CacheKey::CachePositions.get_key(&client_id);
        let mut conn = self.redis_conn.get_multiplexed_async_connection().await?;
        let account_json: Option<String> = conn.get(&cache_key).await.ok();
        if let Some(account_json) = account_json {
            let pos: Vec<positions::Model> = serde_json::from_str(&account_json)?;
            return Ok(pos);
        }
        Ok(vec![])
    }
}