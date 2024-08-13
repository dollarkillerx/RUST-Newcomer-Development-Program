use http::{http_request::HttpRequest, http_response::HttpResponse};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::env;
use tokio::fs;

#[async_trait::async_trait]
pub trait Handler {
    // 处理请求
    async fn handle(req: HttpRequest) -> HttpResponse;

    // 读取文件
    async fn load_file(file_name: &str) -> Option<String> {
        let default_path = format!("{}/public", env!("CARGO_MANIFEST_DIR"));
        let public_path = env::var("PUBLIC_PATH").unwrap_or(default_path);
        let full_path = format!("{}/{}", public_path, file_name);
        fs::read_to_string(full_path).await.ok()
    }
}

pub struct StaticPageHandler;
pub struct WebServiceHandler;
pub struct PageNotFoundHandler;

#[derive(Serialize, Deserialize, Debug)]
pub struct OrderStatus {
    order_id: String,
    order_status: String,
    order_date: String,
}

#[async_trait::async_trait]
impl Handler for PageNotFoundHandler {
    async fn handle(_req: HttpRequest) -> HttpResponse {
        HttpResponse::new("404", None, Self::load_file("404.html").await)
    }
}

#[async_trait::async_trait]
impl Handler for StaticPageHandler {
    async fn handle(_req: HttpRequest) -> HttpResponse {
        let http::http_request::Resource::Path(s) = &_req.resource;
        let route: Vec<&str> = s.split("/").collect();
        match route[1] {
            "" => HttpResponse::new("200", None, Self::load_file("index.html").await),
            "health" => HttpResponse::new("200", None, Self::load_file("health.html").await),
            path => match Self::load_file(path).await {
                Some(contents) => {
                    let mut map: HashMap<&str, &str> = HashMap::new();
                    if path.ends_with(".css") {
                        map.insert("Content-Type", "text/css");
                    } else if path.ends_with(".png") {
                        map.insert("Content-Type", "image/png");
                    } else if path.ends_with(".jpg") {
                        map.insert("Content-Type", "image/jpeg");
                    } else if path.ends_with(".ico") {
                        map.insert("Content-Type", "image/x-icon");
                    } else if path.ends_with(".js") {
                        map.insert("Content-Type", "application/javascript");
                    } else if path.ends_with(".json") {
                        map.insert("Content-Type", "application/json");
                    } else if path.ends_with(".html") {
                        map.insert("Content-Type", "text/html");
                    }

                    HttpResponse::new("200", Some(map), Some(contents))
                }
                None => HttpResponse::new("404", None, Self::load_file("404.html").await),
            },
        }
    }
}

impl WebServiceHandler {
    async fn load_json() -> Vec<OrderStatus> {
        let default_path = format!("{}/data", env!("CARGO_MANIFEST_DIR"));
        let data_path = env::var("DATA_PATH").unwrap_or(default_path);
        let full_path = format!("{}/{}", data_path, "orders.json");
        let json_contents = fs::read_to_string(full_path).await.unwrap();
        let orders: Vec<OrderStatus> = serde_json::from_str(&json_contents).unwrap(); // 反序列化为对象
        orders
    }
}

#[async_trait::async_trait]
impl Handler for WebServiceHandler {
    async fn handle(req: HttpRequest) -> HttpResponse {
        let http::http_request::Resource::Path(s) = &req.resource;
        let route: Vec<&str> = s.split("/").collect();
        // localhost:300/api/shipping/orders
        match route[2] {
            "shipping" if route.len() > 2 && route[3] == "orders" => {
                let body = Some(serde_json::to_string(&Self::load_json().await).unwrap());
                let mut map: HashMap<&str, &str> = HashMap::new();
                map.insert("Content-Type", "application/json");
                HttpResponse::new("200", Some(map), body)
            }
            _ => HttpResponse::new("404", None, Self::load_file("404.html").await),
        }
    }
}
