use salvo::prelude::*;
use salvo::affix;
use crate::api::ea::broadcast;
use crate::api::health::health;
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
                Router::new().post(broadcast)
            ))
            .push(Router::with_path("/subscription").push(
                Router::new().post(broadcast)
            )),
    ).push(
        Router::with_path("/api")
            .push(Router::with_path("/accounts").push(
                Router::new().get(broadcast)
            ))
            .push(Router::with_path("/account/<account>").push(
                Router::new().get(broadcast)
            ))
            .push(Router::with_path("/account/charts/<account>").push(
                Router::new().get(broadcast)
            )),
    );

    router
}