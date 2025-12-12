use std::sync::Arc;

use futures::future::BoxFuture;

use crate::{context::Context, request::HttpRequest, responder::Responder};

pub type Handler = Arc<
    dyn Fn(Arc<HttpRequest>, Arc<Context>) -> BoxFuture<'static, Box<dyn Responder + Send>>
        + Send
        + Sync
        + 'static,
>;

pub trait IntoHandler {
    fn into_handler(self) -> Handler;
}

impl<F, Fut, R> IntoHandler for F
where
    F: Fn(Arc<HttpRequest>, Arc<Context>) -> Fut + Send + Sync + 'static,
    Fut: Future<Output = R> + Send + 'static,
    R: Responder + Send + 'static,
{
    fn into_handler(self) -> Handler {
        Arc::new(move |req: Arc<HttpRequest>, ctx: Arc<Context>| {
            let fut = (self)(req, ctx);
            Box::pin(async move {
                let res = fut.await;
                Box::new(res) as Box<dyn Responder + Send>
            })
        })
    }
}

impl IntoHandler for Handler {
    fn into_handler(self) -> Handler {
        self
    }
}
