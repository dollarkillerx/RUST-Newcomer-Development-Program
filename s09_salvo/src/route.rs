use crate::api::accounts;
use crate::api::broadcast;
use crate::api::close_signal;
use crate::api::health::health;
use crate::api::subscription;
use crate::AppState;
use salvo::affix;
use salvo::prelude::*;

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
                Router::new().post(broadcast::broadcast)
            ))
            .push(Router::with_path("/subscription").push(
                Router::new().post(subscription::subscription)
            ))
            .push(Router::with_path("/close_signal/<client_id>").push(
                Router::new().get(close_signal::get_close_signal)
            ))
            .push(Router::with_path("/close_signal/<client_id>").push(
                Router::new().post(close_signal::set_close_signal)
            ))
    ).push(
        Router::with_path("/api")
            .push(Router::with_path("/accounts").push(
                Router::new().get(accounts::accounts)
            ))
            .push(Router::with_path("/account/<account>").push(
                Router::new().get(accounts::account)
            ))
            .push(Router::with_path("/account/charts/<account>").push(
                Router::new().get(accounts::account_charts)
            )),
    );

    router
}