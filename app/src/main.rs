use milim_web::async_trait;
use milim_web::fairing::Fairing;
use milim_web::request::Method::*;
use milim_web::{context::Context, request::HttpRequest, response::HttpResponse, run_app, server};

fn hello(req: &HttpRequest, res: &mut HttpResponse, ctx: &Context) {
    res.body("Hello World!!");
}
pub struct Log;

impl Log {
    fn new() -> Self {
        Self
    }
}

#[async_trait]
impl Fairing for Log {
    // Deve retornar true para passar para o proximo middleware ou proxima rota exemplo:
    // M1 -> M2 -> Rota -> M2 -> M1
    async fn on_request(&self, req: &mut HttpRequest, res: &mut HttpResponse, ctx: &Context) {
        println!("request method: {:?}", req.raw.method);
    }

    async fn on_response(&self, req: &HttpRequest, res: &mut HttpResponse, ctx: &Context) {
        println!("Response body {:?}", res.body);
    }
}

fn main() {
    let mut app = server();

    app.route(Get, "/hello").handler(hello);

    run_app(|| async {
        let _ = app.listen("127.0.0.1:3000").await;
    });
}
