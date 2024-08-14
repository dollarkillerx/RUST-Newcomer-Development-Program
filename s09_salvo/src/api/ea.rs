use redis::{AsyncCommands, Commands};
use salvo::prelude::*;
use req::BroadcastPayload;
use crate::{ AppState};
use crate::errors::CustomError;
use crate::models::req;

#[handler]
pub async fn broadcast(req: &mut Request,res: &mut Response, depot: &mut Depot) -> Result<(), CustomError> {
    let state =depot.obtain::<AppState>().unwrap();
    let mut conn = state.redis.get_multiplexed_async_connection().await?;
    let broadcast_payload = req.parse_json::<BroadcastPayload>().await?;
    // conn.set_ex()
    res.render(Json(&broadcast_payload));
    Ok(())
}