use std::sync::Arc;

use crate::{
    context::Context,
    request::{HttpRequest, Method},
    response::HttpResponse,
};

pub type Handler = Arc<dyn Fn(&HttpRequest, &mut HttpResponse, &Context) + Send + Sync + 'static>;

pub trait Middleware: Send + Sync + 'static {
    fn on_request(&self, req: &mut HttpRequest, ctx: &Context) -> bool;
    fn on_response(&self, req: &HttpRequest, res: &mut HttpResponse, ctx: &Context);
}

pub struct Router {
    pub(crate) pattern: String,
    pub(crate) handler: Handler,
    pub(crate) method: Method,
    pub(crate) route_middlewares: Vec<Arc<dyn Middleware>>,
}
impl Router {
    pub fn new(
        pattern: &str,
        handler: Handler,
        method: Method,
        middlewares: Vec<Arc<dyn Middleware>>,
    ) -> Self {
        Router {
            pattern: pattern.to_string(),
            handler,
            method,
            route_middlewares: middlewares,
        }
    }
}
