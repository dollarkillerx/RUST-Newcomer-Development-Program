use salvo::prelude::*;
use req::BroadcastPayload;
use crate::AppState;
use crate::models::req;

#[handler]
pub async fn broadcast(req: &mut Request,res: &mut Response, depot: &mut Depot) -> Result<()> {
    let state = depot
        .obtain::<AppState>()
        .ok_or_else(StatusError::internal_server_error)?;
    let broadcast_payload = req.parse_json::<BroadcastPayload>().await.ok()?;

    Ok(())
}