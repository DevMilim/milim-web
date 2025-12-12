use std::{fmt::Debug, sync::Arc};

use crate::{
    aplication::App,
    context::Context,
    guard::{Guard, IntoGuard},
    request::{HttpRequest, Method},
    responder::Responder,
    response::HttpResponse,
};

pub trait IntoBody: Debug {
    fn into_body(self) -> String;
}

impl IntoBody for String {
    fn into_body(self) -> String {
        self
    }
}

impl IntoBody for &str {
    fn into_body(self) -> String {
        self.to_string()
    }
}

pub type Handler = Arc<
    dyn Fn(&HttpRequest, &Context) -> Box<dyn Responder + Send + 'static> + Send + Sync + 'static,
>;

pub trait IntoHandler {
    fn into_handler(self) -> Handler;
}

impl<F, R> IntoHandler for F
where
    F: Fn(&HttpRequest, &Context) -> R + Send + Sync + 'static,
    R: Responder + Send + 'static,
{
    fn into_handler(self) -> Handler {
        Arc::new(move |req: &HttpRequest, ctx: &Context| {
            let res = (self)(req, ctx);
            Box::new(res) as Box<dyn Responder + Send>
        })
    }
}

impl IntoHandler for Handler {
    fn into_handler(self) -> Handler {
        self
    }
}

pub struct RouteBuilder<'a> {
    pub(crate) app: &'a mut App,
    pub(crate) pattern: String,
    pub(crate) method: Method,
    pub(crate) guards: Vec<Arc<dyn Guard>>,
}

impl<'a> RouteBuilder<'a> {
    pub fn faiting<M: IntoGuard>(mut self, guard: M) -> Self {
        self.guards.push(guard.into_guard());
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
            guards: self.guards,
        });
        app_ref
    }
}

#[derive(Clone)]
pub struct Router {
    pub(crate) pattern: String,
    pub(crate) handler: Handler,
    pub(crate) method: Method,
    pub(crate) guards: Vec<Arc<dyn Guard>>,
}
impl Router {
    pub fn new(
        pattern: &str,
        handler: Handler,
        method: Method,
        middlewares: Vec<Arc<dyn Guard>>,
    ) -> Self {
        Router {
            pattern: pattern.to_string(),
            handler,
            method,
            guards: middlewares,
        }
    }
}
