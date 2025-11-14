use std::sync::Arc;

use crate::{
    aplication::App,
    context::Context,
    request::{HttpRequest, Method},
    response::HttpResponse,
};

pub type Handler = Arc<dyn Fn(&HttpRequest, &mut HttpResponse, &Context) + Send + Sync + 'static>;

pub trait IntoHandler {
    fn into_handler(self) -> Handler;
}

impl<F> IntoHandler for F
where
    F: Fn(&HttpRequest, &mut HttpResponse, &Context) + Send + Sync + 'static,
{
    fn into_handler(self) -> Handler {
        Arc::new(self)
    }
}

impl IntoHandler for Handler {
    fn into_handler(self) -> Handler {
        self
    }
}

pub trait Middleware: Send + Sync + 'static {
    fn on_request(&self, req: &mut HttpRequest, ctx: &Context) -> MwFlow;
    fn on_response(&self, req: &HttpRequest, res: &mut HttpResponse, ctx: &Context);
}

pub trait IntoMiddleware {
    fn into_middleware(self) -> Arc<dyn Middleware>;
}

impl<T> IntoMiddleware for T
where
    T: Middleware + Send + Sync + 'static,
{
    fn into_middleware(self) -> Arc<dyn Middleware> {
        Arc::new(self)
    }
}

impl IntoMiddleware for Arc<dyn Middleware> {
    fn into_middleware(self) -> Arc<dyn Middleware> {
        self
    }
}

#[derive(PartialEq)]
pub enum MwFlow {
    Continue,
    Stop,
}

pub struct RouteBuilder<'a> {
    pub(crate) app: &'a mut App,
    pub(crate) pattern: String,
    pub(crate) method: Method,
    pub(crate) middlewares: Vec<Arc<dyn Middleware>>,
}

impl<'a> RouteBuilder<'a> {
    pub fn with_middleware<M: IntoMiddleware>(mut self, middleware: M) -> Self {
        self.middlewares.push(middleware.into_middleware());
        self
    }
    pub fn handler<H>(self, handler: H) -> &'a mut App
    where
        H: IntoHandler,
    {
        let app_ref: &mut App = self.app;
        app_ref.add_route(Router {
            pattern: self.pattern,
            handler: handler.into_handler(),
            method: self.method,
            middlewares: self.middlewares,
        });
        app_ref
    }
}

pub struct Router {
    pub(crate) pattern: String,
    pub(crate) handler: Handler,
    pub(crate) method: Method,
    pub(crate) middlewares: Vec<Arc<dyn Middleware>>,
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
            middlewares: middlewares,
        }
    }
}
