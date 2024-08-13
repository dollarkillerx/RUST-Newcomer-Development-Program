use super::handler::{Handler, PageNotFoundHandler, StaticPageHandler, WebServiceHandler};
use http::{ http_request, http_request::HttpRequest, http_response::HttpResponse,http_response };
use tokio::io::{ AsyncWriteExt};

pub struct Router;

impl Router {
    pub async fn router(req: HttpRequest, stream: &mut (impl AsyncWriteExt + Unpin)) -> () {
        match req.method {
            http_request::Method::Get => match &req.resource {
                http_request::Resource::Path(path) => {
                    let route: Vec<&str> = path.split("/").collect();
                    match route[1] {
                        "api" => {
                            let resp = WebServiceHandler::handle(req.clone()).await;
                            let _ = resp.send_response(stream).await;
                        }
                        _ => {
                            let resp = StaticPageHandler::handle(req.clone()).await;
                            let _ = resp.send_response(stream).await;
                        }
                    }
                }
            }
            _ => {
                let resp = PageNotFoundHandler::handle(req.clone()).await;
                let _ = resp.send_response(stream).await;
            }
        }
    }
}