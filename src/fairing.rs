use std::sync::Arc;

use async_trait::async_trait;

use crate::{context::Context, request::HttpRequest, response::HttpResponse};

#[async_trait]
pub trait Fairing: Send + Sync + 'static {
    async fn on_request(&self, req: &mut HttpRequest, ctx: &Context);
    async fn on_response(&self, req: &HttpRequest, res: &mut HttpResponse, ctx: &Context);
}

pub trait IntoFairing {
    fn into_fairing(self) -> Arc<dyn Fairing>;
}

impl<T> IntoFairing for T
where
    T: Fairing + Send + Sync + 'static,
{
    fn into_fairing(self) -> Arc<dyn Fairing> {
        Arc::new(self)
    }
}

impl IntoFairing for Arc<dyn Fairing> {
    fn into_fairing(self) -> Arc<dyn Fairing> {
        self
    }
}
