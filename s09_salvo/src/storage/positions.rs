use sea_orm::prelude::Decimal;
use futures::stream::{self, StreamExt};use tracing_subscriber::fmt::format;
use crate::entity::positions;
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
                    opening_time_system: opening_time_system,
                    closing_time_system: Option::from(x.closing_time_system),
                })
            })
            .collect()
            .await;

        Ok(())
    }
}