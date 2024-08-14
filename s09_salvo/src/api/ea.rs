use redis::{AsyncCommands, };
use salvo::prelude::*;
use serde_json::json;
use req::BroadcastPayload;
use crate::{ AppState};
use crate::entity::account;
use crate::errors::CustomError;
use crate::models::req;
use crate::models::req::CacheKey;

#[handler]
pub async fn broadcast(req: &mut Request, res: &mut Response, depot: &mut Depot) -> Result<(), CustomError> {
    let state = depot.obtain::<AppState>().unwrap();
    let mut conn = state.redis.get_multiplexed_async_connection().await?;
    let broadcast_payload = req.parse_json::<BroadcastPayload>().await?;

    // 创建并存储 account_entity
    let account_entity = broadcast_payload.to_account_entity()?;
    let cache_key = CacheKey::CacheAccount.get_key(&broadcast_payload.client_id);
    let account_json = serde_json::to_string(&account_entity)?;

    // 存储到 redis，有效期为一年
    const ONE_YEAR_IN_SECONDS: u64 = 365 * 24 * 60 * 60;
    conn.set_ex(&cache_key, &account_json, ONE_YEAR_IN_SECONDS).await?;

    // 从 redis 获取并反序列化 account_entity
    let stored_account: account::Model = conn.get(&cache_key).await
        .map_err(CustomError::from)
        .and_then(|json: String| serde_json::from_str(&json).map_err(CustomError::from))?;

    res.render(Json(&stored_account));
    Ok(())
}