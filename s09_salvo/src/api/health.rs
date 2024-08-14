use chrono::Local;
use maplit::hashmap;
use salvo::prelude::*;

#[handler]
pub async fn health(res: &mut Response) {
    // 使用 Json 渲染 map 作为响应
    res.render(Json(hashmap!{
        "status" => "ok".to_string(),
        "time" => Local::now().format("%Y-%m-%d %H:%M:%S").to_string()
    }));
}
