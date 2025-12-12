use std::{fmt::Debug, sync::Arc};

use crate::{
    aplication::App,
    guard::{Guard, IntoGuard},
    handler::{Handler, IntoHandler},
    request::Method,
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
