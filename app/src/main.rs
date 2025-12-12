use milim_web::{context::Context, macros::handler, request::HttpRequest, run_app, server};

#[handler]
async fn hello(req: &HttpRequest, ctx: &Context) -> &'static str {
    "Hello World!!"
}

fn main() {
    use milim_web::request::Method::*;
    let mut app = server();

    app.route(Get, "/hello").handler(hello);

    run_app(|| async {
        let _ = app.listen("127.0.0.1:3000").await;
    });
}
