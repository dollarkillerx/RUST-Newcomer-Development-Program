use ntex::service::{Middleware, Service, ServiceCtx};
use ntex::web;

pub struct SayHi;

impl Default for SayHi {
    fn default() -> Self {
        SayHi
    }
}

impl<S> Middleware<S> for SayHi {
    type Service = SayHiMiddleware<S>;

    fn create(&self, service: S) -> Self::Service {
        SayHiMiddleware { service }
    }
}

pub struct SayHiMiddleware<S> {
    service: S,
}

impl<S, Err> Service<web::WebRequest<Err>> for SayHiMiddleware<S>
where
    S: Service<web::WebRequest<Err>, Response = web::WebResponse, Error = web::Error>,
    Err: web::ErrorRenderer,
{
    type Response = web::WebResponse;
    type Error = web::Error;

    ntex::forward_ready!(service);

    async fn call(&self, req: web::WebRequest<Err>, ctx: ServiceCtx<'_, Self>) -> Result<Self::Response, Self::Error> {
        println!("Hi from start. You requested: {}", req.path());
        let res = ctx.call(&self.service, req).await?;
        println!("Hi from response");
        Ok(res)
    }
}