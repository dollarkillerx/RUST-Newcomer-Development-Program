use super::*;
use once_cell::sync::OnceCell;
use tokio::sync::Mutex;
use std::collections::HashMap;

fn global_data() -> &'static Mutex<HashMap<String, String>> {
    static INSTANCE: OnceCell<Mutex<HashMap<String, String>>> = OnceCell::new();
    INSTANCE.get_or_init(|| {
        let mut m = HashMap::new();
        m.insert("dollarkiller".to_string(), "123456".to_string());
        Mutex::new(m)
    })
}

#[fn_handler]
pub async fn index(res: &mut Response) {
    println!("index: hello world!");
    res.render_plain_text("Hello world!");
}

#[fn_handler]
pub async fn auth(req: &mut Request, depot: &mut Depot, res: &mut Response) {
    println!("auth: user has authed");
    // let user: Option<String> = req.get_query("user");
    // let password: Option<String> = req.get_query("password");

    let user: String = req.get_query("in_user").unwrap_or("".to_string());
    let password: String = req.get_query("in_password").unwrap_or("".to_string());

    if user != "dollarkiller" && password != "123456" {
        res.set_status_code(StatusCode::BAD_REQUEST);
        res.render_plain_text("400");
        return;
    }

    let user: Option<String> = req.get_query("user");
    let password: Option<String> = req.get_query("password");

    depot.insert("user", user);
    depot.insert("password", password);
    res.render_plain_text("user has authed\n\n");
}

#[fn_handler]
pub async fn create_user(depot: &mut Depot, res: &mut Response) {
    println!("create_user: user created");
    let user: Option<String> = depot.take("user");
    let password: Option<String> = depot.take("password");

    let user = user.unwrap_or("".to_string());
    let password = password.unwrap_or("".to_string());

    global_data().lock().await.insert(user.clone(), password.clone());
    res.render_plain_text(format!("user created user: {} password: {}", user, password).as_str());
}

#[fn_handler]
pub async fn update_user(res: &mut Response) {
    println!("update_user: user updated");
    res.render_plain_text("user updated");
}

#[fn_handler]
pub async fn delete_user(res: &mut Response) {
    println!("delete_user: user deleted");
    res.render_plain_text("user deleted");
}

#[fn_handler]
pub async fn list_users(res: &mut Response) {
    println!("list_users: list users");

    let rc = format!("list users {:?}", global_data().lock().await);
    res.render_plain_text(rc.as_str());
}

// /users/<id: num>
#[fn_handler]
pub async fn show_user(req: &mut Request, res: &mut Response) {
    println!("show_user: show user");
    let id: Option<i32> = req.get_param("id");
    res.render_plain_text(format!("show user {:?}", id).as_str());
}

#[fn_handler]
pub async fn admin(res: &mut Response) {
    println!("admin: Admin page");
    res.render_plain_text("Admin page");
}

#[fn_handler]
pub async fn debug(res: &mut Response) {
    println!("debug: Debug page");
    res.render_plain_text("Debug page");
}
