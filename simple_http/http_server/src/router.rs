use std::sync::Arc;
// use super::handler::{Handler, PageNotFoundHandler, StaticPageHandler, WebServiceHandler};
// use http::{ http_request, http_request::HttpRequest, http_response::HttpResponse,http_response };
use http::{ http_request, http_request::HttpRequest};
use tokio::io::{ AsyncWriteExt};
use tokio::net::TcpStream;

pub struct Router;

impl Router {
    pub async fn router(req: HttpRequest, stream: Arc<&TcpStream>) -> () {
        println!("this is router");
        match req.method {
            http_request::Method::Get => match &req.resource {
                http_request::Resource::Path(path) => {
                    let route: Vec<&str> = path.split("/").collect();
                    println!("{:?}", route);
                    // match route[1] {
                    //     "api" => {
                    //         let resp = WebServiceHandler::handle(req.clone()).await;
                    //         let _ = resp.send_response(stream).await;
                    //     }
                    //     _ => {
                    //         let resp = StaticPageHandler::handle(req.clone()).await;
                    //         let _ = resp.send_response(stream).await;
                    //     }
                    // }
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