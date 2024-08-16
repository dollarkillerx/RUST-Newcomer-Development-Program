use salvo::prelude::*;
use crate::app_state::AppState;
use crate::errors::CustomError;
use crate::models::req::RespResult;

#[handler]
pub async fn accounts(req: &mut Request, res: &mut Response, depot: &mut Depot) -> Result<(), CustomError> {
    let state = depot.obtain::<AppState>().unwrap();
    let accts = state.storage.get_accounts().await?;

    res.render(Json(RespResult::new(
        200,
        "ok".to_owned(),
        accts,
    )));
    Ok(())
}

#[handler]
pub async fn account(req: &mut Request, res: &mut Response, depot: &mut Depot) -> Result<(), CustomError> {
    let account_param = req.param::<String>("account").ok_or(CustomError::ParamError("error".into()))?;

    let state = depot.obtain::<AppState>().unwrap();
    let accts = state.storage.get_accounts().await?;

    for acct in accts {
        if acct.client_id == account_param {
            res.render(Json(RespResult::new(
                200,
                "ok".to_owned(),
                acct,
            )));
            return Ok(());
        }
    }

    Ok(())
}

#[handler]
pub async fn account_charts(req: &mut Request, res: &mut Response, depot: &mut Depot) -> Result<(), CustomError> {
    let account_param = req.param::<String>("account").ok_or(CustomError::ParamError("error".into()))?;
    let state = depot.obtain::<AppState>().unwrap();



    Ok(())
}
