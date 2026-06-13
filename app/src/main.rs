use std::sync::Mutex;

use milim_web::{context::Context, macros::handler, request::HttpRequest, run_app, server};

#[handler]
async fn hello(req: &HttpRequest, ctx: &Context) -> String {
    let mut count = ctx.get_state::<Mutex<u32>>().unwrap().lock().unwrap();

    *count += 1;

    format!("Hello: {}", count)
}

fn main() {
    use milim_web::request::Method::*;
    let mut app = server();

    app.manage(Mutex::new(0u32));

    app.route(Get, "/").handler(hello);

    run_app(|| async {
        let _ = app.listen("127.0.0.1:3000").await;
    });
}
