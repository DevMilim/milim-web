use milim_web::{context::Context, request::HttpRequest, response::HttpResponse, run_app, server};

fn hello(req: &HttpRequest, res: &mut HttpResponse, ctx: &Context) {
    res.body("Hello World!!");
}

fn main() {
    use milim_web::request::Method::*;
    let mut app = server();

    let _a = async |req: &HttpRequest, res: &mut HttpResponse, ctx: &Context| {
        res.body("Hello World!!");
    };

    app.route(Get, "/hello").handler(hello);

    run_app(|| async {
        let _ = app.listen("127.0.0.1:3000").await;
    });
}
