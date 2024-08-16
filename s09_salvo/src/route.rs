use salvo::prelude::*;
use salvo::affix;
use crate::api::accounts::{account, account_charts, accounts};
use crate::api::broadcast::broadcast;
use crate::api::health::health;
use crate::api::subscription::subscription;
use crate::AppState;

pub fn route(app_state: AppState) -> Router {
    let router = Router::new().hoop(affix::inject(app_state)).push(
        Router::with_path("/api")
            .push(
                Router::with_path("/health")
                    .push(Router::new().get(health))
            ),
    ).push(
        Router::with_path("/ea")
            .push(Router::with_path("/broadcast").push(
                    Router::new().post(broadcast)
            ))
            .push(Router::with_path("/subscription").push(
                Router::new().post(subscription)
            ))
    ).push(
        Router::with_path("/api")
            .push(Router::with_path("/accounts").push(
                Router::new().get(accounts)
            ))
            .push(Router::with_path("/account/<account>").push(
                Router::new().get(account)
            ))
            .push(Router::with_path("/account/charts/<account>").push(
                Router::new().get(account_charts)
            )),
    );

    router
}