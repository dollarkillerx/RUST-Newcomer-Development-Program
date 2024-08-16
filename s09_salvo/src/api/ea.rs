use tracing::info;
use salvo::prelude::*;
use req::BroadcastPayload;
use crate::{AppState};
use crate::entity::account;
use crate::errors::CustomError;
use crate::models::req;

#[handler]
pub async fn broadcast(req: &mut Request, res: &mut Response, depot: &mut Depot) -> Result<(), CustomError> {
    let state = depot.obtain::<AppState>().unwrap();

    let broadcast_payload = req.parse_json::<BroadcastPayload>().await?;

    // 创建并存储 account_entity
    let account_entity = broadcast_payload.to_account_entity()?;
    // 更新 account_entity
    state.storage.update_account(&account_entity).await?;
    // 更新 positions
    state.storage.update_positions(&account_entity.client_id, &broadcast_payload.positions).await?;
    // 更新 history
    state.storage.update_history(&account_entity.client_id, &broadcast_payload.history).await?;

    let stored_account: account::Model = state.storage.get_account(&account_entity.client_id).await?;

    res.render(Json(&stored_account));
    Ok(())
}
