use std::sync::{Arc};
use http::{http_request, http_request::HttpRequest};
use tokio::net::TcpStream;
use crate::handler::{Handler, PageNotFoundHandler, StaticPageHandler, WebServiceHandler};

pub struct Router;

impl Router {
    pub async fn router(req: HttpRequest, socket:  &mut TcpStream) {
        let req = Arc::new(req);
        match req.method {
            http_request::Method::Get => match &req.resource {
                http_request::Resource::Path(path) => {
                    let route: Vec<&str> = path.split("/").collect();
                    println!("{:?}", route);
                    match route[1] {
                        "api" => {
                            let resp = WebServiceHandler::handle(req).await;
                            let _ = resp.send_response(socket).await;
                        }
                        _ => {
                            let resp = StaticPageHandler::handle(req).await;
                            let _ = resp.send_response(socket).await;
                        }
                    }
                }
            }
            _ => {
                let resp = PageNotFoundHandler::handle(req).await;
                let _ = resp.send_response(socket).await;
            }
        }
    }
}