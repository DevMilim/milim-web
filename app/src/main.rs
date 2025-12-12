use std::sync::Arc;

use milim_web::async_trait;
use milim_web::fairing::Fairing;
use milim_web::request::Method::*;
use milim_web::responder::Responder;
use milim_web::{context::Context, request::HttpRequest, response::HttpResponse, run_app, server};

pub struct Log;

impl Log {
    fn new() -> Self {
        Self
    }
}

#[async_trait]
impl Fairing for Log {
    // Deve retornar true para passar para o proximo Fairing ou proxima rota exemplo:
    // M1 -> M2 -> Rota -> M2 -> M1
    async fn on_request(&self, req: &mut HttpRequest, ctx: &Context) {
        req.ctx.data(12);
        println!("request method: {:?}", req.raw.method);
    }

    async fn on_response(&self, req: &HttpRequest, res: &mut HttpResponse, ctx: &Context) {
        println!("Count {:?}", req.ctx.get::<i32>());
    }
}

fn index(req: &HttpRequest, ctx: &Context) -> &'static str {
    ""
}

fn main() {
    let mut app = server();
    app.fairing(Log::new());

    app.route(Get, "/").handler(index);

    run_app(|| async {
        let _ = app.listen("127.0.0.1:3000").await;
    });
}
