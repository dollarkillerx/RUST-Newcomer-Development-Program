use super::*;

pub fn router() -> Router {

    // return Router::new().get(server::index);

    let debug_mode = true;
    let admin_mode = true;

    Router::new()
        .get(server::index)
        .push(
            Router::new()
                .path("users")
                .before(server::auth)
                .post(server::create_user)
                .push(Router::new().path(r"<id:num>").
                    post(server::update_user)
                    .delete(server::delete_user)),
        )

        .push(
            Router::new()
                .path("users")
                .get(server::list_users)
                .push(
                    Router::new()
                        .path(r"<id:num>")
                        .get(server::show_user)
                ),
        )
        .then(|router| {
            if debug_mode {
                router.push(Router::new().path("debug").get(server::debug))
            } else {
                router
            }
        })
        .then(|router| {
            if admin_mode {
                router.push(Router::new().path("admin").get(server::admin))
            } else {
                router
            }
        })
}