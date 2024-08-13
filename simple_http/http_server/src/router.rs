use std::sync::{Arc};
use tokio::io::AsyncWriteExt;
use tokio::sync::Mutex;
use http::{http_request, http_request::HttpRequest};
use tokio::net::TcpStream;
use http::http_response::HttpResponse;

pub struct Router;


impl Router {
    pub async fn router(req: HttpRequest, socket: Arc<Mutex<TcpStream>>) {
        match req.method {
            http_request::Method::Get => match &req.resource {
                http_request::Resource::Path(path) => {
                    let route: Vec<&str> = path.split("/").collect();
                    println!("{:?}", route);
                    match route[1] {
                        "api" => {
                            // let resp = WebServiceHandler::handle(req.clone()).await;
                            // let _ = resp.send_response(stream).await;
                        }
                        _ => {
                            let resp = HttpResponse::default();
                            resp.send_response(socket).await.unwrap();
                            // let resp = StaticPageHandler::handle(req).await;
                        }
                    }
                }
            }
            _ => {
                println!("{:?}", req.method);
                // let resp = PageNotFoundHandler::handle(req.clone()).await;
                // let _ = resp.send_response(stream).await;
            }
        }
    }
}