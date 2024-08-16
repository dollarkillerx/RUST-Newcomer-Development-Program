use salvo::prelude::*;
use crate::{AppState};
use crate::errors::CustomError;
use crate::models::req::{Positions, SubscriptionPayload, SubscriptionResponse};

pub async fn subscription(req: &mut Request, res: &mut Response, depot: &mut Depot) -> Result<(), CustomError> {
    let state = depot.obtain::<AppState>().unwrap();

    let broadcast_payload = req.parse_json::<SubscriptionPayload>().await?;


    // 存储基础information
    // 创建并存储 account_entity
    let account_entity = broadcast_payload.to_account_entity()?;
    // 更新 account_entity
    state.storage.update_account(&account_entity).await?;
    // 更新 positions
    state.storage.update_positions(&account_entity.client_id, &broadcast_payload.positions).await?;
    // 更新 history
    state.storage.update_history(&account_entity.client_id, &broadcast_payload.history).await?;
    // 存储基础information end

    let mut result: SubscriptionResponse = SubscriptionResponse {
        client_id: "".to_string(),
        subscription_client_id: "".to_string(),
        open_positions: vec![],
        close_position: vec![],
    };
    // 获取订阅信息
    let open_positions = state.storage.get_positions(&broadcast_payload.subscription_client_id).await?;
    let close_position = state.storage.get_history(&broadcast_payload.subscription_client_id).await?;

    let open_positions:Vec<Positions> = open_positions.iter().map(|x| {
        Positions {
            order_id: x.order_id.clone(),
            direction: x.direction.clone(),
            price: x.price.clone(),
            volume: x.volume.clone(),
            created_at: x.created_at.clone(),
        }
    }).collect();
    let close_position:Vec<Positions> = close_position.iter().map(|x| x).collect();

    result.open_positions = open_positions;
    result.close_position = close_position;

    res.render(Json(&result));
    Ok(())
}