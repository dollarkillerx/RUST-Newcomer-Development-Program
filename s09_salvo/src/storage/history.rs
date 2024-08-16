use std::collections::HashMap;
use std::time::{SystemTime, UNIX_EPOCH};
use chrono::Utc;
use sea_orm::prelude::Decimal;
use futures::stream::{self, StreamExt};
use redis::AsyncCommands;
use crate::entity::{history};
use crate::enums::enums::CacheKey;
use crate::errors::CustomError;
use crate::models::req::History;
use crate::storage::storage::Storage;

impl Storage {
    pub async fn update_history(&self, client_id: &str, history_input: &Vec<History>) -> Result<(), CustomError> {
        let mut history_map: HashMap<i64, Vec<&History>> = HashMap::new();
        // 分组历史记录
        history_input.iter().for_each(|x| {
            history_map.entry(x.position_id).or_insert_with(Vec::new).push(x);
        });

        // 过滤并创建模型数据
        let pos: Vec<history::Model> = futures::stream::iter(history_map.values())
            .filter_map(|histories| async move {
                if histories.len() != 2 {
                    if histories[0].ticket == histories[0].position_id {
                        return None;
                    }

                    // 创建单个开仓记录的 Position
                    let position = self.create_position_from_history(client_id, histories[0], None).await.ok()?;
                    return Some(position);
                }

                // 按时间排序
                let mut sorted_histories = histories.clone();
                sorted_histories.sort_by_key(|h| h.time_setup);

                let opening = sorted_histories[0];
                let closing = sorted_histories[1];

                // 创建开仓和平仓记录的 Position
                let position = self.create_position_from_history(client_id, opening, Some(closing)).await.ok()?;
                Some(position)
            })
            .collect()
            .await;

        // 存储数据到 Redis，缓存时间为 30 天
        let cache_key = CacheKey::CacheHistory.get_key(client_id);
        let account_json = serde_json::to_string(&pos)?;
        let mut conn = self.redis_conn.get_multiplexed_async_connection().await?;
        const ONE_MONTH_IN_SECONDS: u64 = 30 * 24 * 60 * 60;
        conn.set_ex(&cache_key, &account_json, ONE_MONTH_IN_SECONDS).await?;
        Ok(())
    }

    // 辅助函数：根据历史记录创建 Position
    async fn create_position_from_history(&self,
                                          client_id: &str,
                                          opening: &History,
                                          closing: Option<&History>,
    ) -> Result<history::Model, CustomError> {
        let now = Utc::now();

        let opening_time_system = self.gen_time(format!("{}_{}_{}", client_id, "openingTimeSystem", opening.ticket).as_str()).await?;

        let profit = match closing {
            Some(cl) => Decimal::from_f64_retain(cl.price_current - opening.price_current).unwrap(),
            None => Decimal::ZERO,
        };

        Ok(history::Model {
            id: xid::new().to_string(),
            created_at: Some(now.into()),
            updated_at: Some(now.into()),
            deleted_at: None,
            client_id: client_id.to_string(),
            order_id: opening.ticket,
            direction: opening.r#type.clone(),
            symbol: opening.symbol.clone(),
            magic: opening.magic,
            open_price: Decimal::from_f64_retain(opening.price_current).unwrap(),
            volume: Decimal::from_f64_retain(opening.volume_initial).unwrap(),
            market: Decimal::from_f64_retain(closing.map_or(opening.price_current, |c| c.price_current)).unwrap(),
            swap: Decimal::ZERO,
            profit,
            common: "".to_owned(),
            opening_time: opening.time_setup,
            closing_time: closing.map_or(opening.time_setup, |c| c.time_setup) as i64,
            common_internal: None,
            opening_time_system: Some(opening_time_system),
            closing_time_system: Some(SystemTime::now().duration_since(UNIX_EPOCH)?.as_secs() as i64),
        })
    }
}