``` 
pub struct StaticPageHandler;
// impl StaticPageHandler {
//     pub async fn handle(req: Arc<HttpRequest>) -> HttpResponse  {
//         let http::http_request::Resource::Path(s) = &req.resource;
//         let route: Vec<&str> = s.split("/").collect();
//         match route[1] {
//             "" => HttpResponse::new("200".to_owned(), None, Self::load_file("index.html").await),
//             "health" => HttpResponse::new("200".to_owned(), None, Self::load_file("health.html").await),
//             path => match Self::load_file(path).await {
//                 Some(contents) => {
//                     let mut map: HashMap<String, String> = HashMap::new();
//                     if path.ends_with(".css") {
//                         map.insert("Content-Type".to_owned(), "text/css".to_owned());
//                     } else if path.ends_with(".png") {
//                         map.insert("Content-Type".to_owned(), "image/png".to_owned());
//                     } else if path.ends_with(".jpg") {
//                         map.insert("Content-Type".to_owned(), "image/jpeg".to_owned());
//                     } else if path.ends_with(".ico") {
//                         map.insert("Content-Type".to_owned(), "image/x-icon".to_owned());
//                     } else if path.ends_with(".js") {
//                         map.insert("Content-Type".to_owned(), "application/javascript".to_owned());
//                     } else if path.ends_with(".json") {
//                         map.insert("Content-Type".to_owned(), "application/json".to_owned());
//                     } else if path.ends_with(".html") {
//                         map.insert("Content-Type".to_owned(), "text/html".to_owned());
//                     }
//
//                     HttpResponse::new("200".to_owned(), Some(map), Some(contents))
//                 }
//                 None => HttpResponse::new("404".to_owned(), None, Self::load_file("404.html").await),
//             },
//         }
//     }
//
//     async fn load_file(file_name: &str) -> Option<String> {
//         let default_path = format!("{}/public", env!("CARGO_MANIFEST_DIR"));
//         let public_path = std::env::var("PUBLIC_PATH").unwrap_or(default_path);
//         let full_path = format!("{}/{}", public_path, file_name);
//         fs::read_to_string(full_path).await.ok()
//     }
// }

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

```


1.75不用恶心的 async_trait 了