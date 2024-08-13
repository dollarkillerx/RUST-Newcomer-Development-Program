use std::collections::HashMap;
use std::env;
use std::sync::Arc;
use serde::{Deserialize, Serialize};
use tokio::fs;
use http::http_request::HttpRequest;
use http::http_response::HttpResponse;

pub struct StaticPageHandler;

pub trait Handler {
    // 处理请求
    async fn handle (req: Arc<HttpRequest>) -> HttpResponse;

    // 读取文件
    async fn load_file(file_name: &str) -> Option<String> {
        let default_path = format!("{}/public", env!("CARGO_MANIFEST_DIR"));
        let public_path = std::env::var("PUBLIC_PATH").unwrap_or(default_path);
        let full_path = format!("{}/{}", public_path, file_name);
        fs::read_to_string(full_path).await.ok()
    }
}

impl Handler for StaticPageHandler {
    async fn handle(req: Arc<HttpRequest>) -> HttpResponse  {
        let http::http_request::Resource::Path(s) = &req.resource;
        let route: Vec<&str> = s.split("/").collect();
        match route[1] {
            "" => HttpResponse::new("200".to_owned(), None, Self::load_file("index.html").await),
            "health" => HttpResponse::new("200".to_owned(), None, Self::load_file("health.html").await),
            path => match Self::load_file(path).await {
                Some(contents) => {
                    let mut map: HashMap<String, String> = HashMap::new();
                    if path.ends_with(".css") {
                        map.insert("Content-Type".to_owned(), "text/css".to_owned());
                    } else if path.ends_with(".png") {
                        map.insert("Content-Type".to_owned(), "image/png".to_owned());
                    } else if path.ends_with(".jpg") {
                        map.insert("Content-Type".to_owned(), "image/jpeg".to_owned());
                    } else if path.ends_with(".ico") {
                        map.insert("Content-Type".to_owned(), "image/x-icon".to_owned());
                    } else if path.ends_with(".js") {
                        map.insert("Content-Type".to_owned(), "application/javascript".to_owned());
                    } else if path.ends_with(".json") {
                        map.insert("Content-Type".to_owned(), "application/json".to_owned());
                    } else if path.ends_with(".html") {
                        map.insert("Content-Type".to_owned(), "text/html".to_owned());
                    }

                    HttpResponse::new("200".to_owned(), Some(map), Some(contents))
                }
                None => HttpResponse::new("404".to_owned(), None, Self::load_file("404.html").await),
            },
        }
    }

}

pub struct WebServiceHandler;
pub struct PageNotFoundHandler;

#[derive(Serialize, Deserialize, Debug)]
pub struct OrderStatus {
    order_id: String,
    order_status: String,
    order_date: String,
}

impl Handler for PageNotFoundHandler {
    async fn handle(_req: Arc<HttpRequest>) -> HttpResponse {
        HttpResponse::new("404".to_owned(), None, Self::load_file("404.html").await)
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

impl Handler for WebServiceHandler {
    async fn handle(req: Arc<HttpRequest>) -> HttpResponse {
        let http::http_request::Resource::Path(s) = &req.resource;
        let route: Vec<&str> = s.split("/").collect();
        // localhost:300/api/shipping/orders
        match route[2] {
            "shipping" if route.len() > 2 && route[3] == "orders" => {
                let body = Some(serde_json::to_string(&Self::load_json().await).unwrap());
                let mut map: HashMap<String, String> = HashMap::new();
                map.insert("Content-Type".to_owned(), "application/json".to_owned());
                HttpResponse::new("200".to_owned(), Some(map), body)
            }
            _ => HttpResponse::new("404".to_owned(), None, Self::load_file("404.html").await),
        }
    }
}
