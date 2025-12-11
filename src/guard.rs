use std::sync::Arc;

use async_trait::async_trait;

use crate::{context::Context, request::HttpRequest, response::HttpResponse};

#[derive(PartialEq, Clone)]
pub enum Outcome {
    Success,
    Failure(HttpResponse),
}

#[async_trait]
pub trait Guard: Send + Sync + 'static {
    async fn from_request(&self, req: &HttpRequest, ctx: &mut Context) -> Outcome;
}

pub trait IntoGuard {
    fn into_guard(self) -> Arc<dyn Guard>;
}

impl<T> IntoGuard for T
where
    T: Guard + Send + Sync + 'static,
{
    fn into_guard(self) -> Arc<dyn Guard> {
        Arc::new(self)
    }
}

impl IntoGuard for Arc<dyn Guard> {
    fn into_guard(self) -> Arc<dyn Guard> {
        self
    }
}
