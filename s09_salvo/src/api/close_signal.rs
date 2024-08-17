use salvo::prelude::*;
use crate::{AppState};
use crate::errors::CustomError;
use crate::models::req;

#[handler]
pub async fn get_close_signal(req: &mut Request, res: &mut Response, depot: &mut Depot) -> Result<(), CustomError> {
    let state = depot.obtain::<AppState>().unwrap();
    let account_id = req.param::<String>("client_id").ok_or(CustomError::ParamError("error".into()))?;

    let ok = state.storage.get_close_signal(&account_id).await?;

    if ok {
        res.render(Json(req::RespResult::new(200, "ok".to_owned(), true)));
    } else {
        res.render(Json(req::RespResult::new(200, "ok".to_owned(), false)));
    }

    Ok(())
}

#[handler]
pub async fn set_close_signal(req: &mut Request, res: &mut Response, depot: &mut Depot) -> Result<(), CustomError> {
    let state = depot.obtain::<AppState>().unwrap();
    let account_id = req.param::<String>("client_id").ok_or(CustomError::ParamError("error".into()))?;
    state.storage.set_close_signal(&account_id).await?;

    res.render(Json(req::RespResult::new(200, "ok".to_owned(), true)));
    Ok(())
}