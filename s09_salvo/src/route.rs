use salvo::Router;
use crate::api::hello::hello;

pub fn route() -> Router{
    let router = Router::new().push(
        Router::with_path("/api")
            .push(
                Router::with_path("/hello").push(  // /api/hello
                    Router::new().get(hello)
                )
            )
    );

    router
}