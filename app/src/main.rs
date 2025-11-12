use milim_web::{
    context::Context, request::HttpRequest, response::HttpResponse, router::Middleware, server,
};

fn user(req: &HttpRequest, res: &mut HttpResponse, ctx: &Context) {
    res.body("funciona");
}

pub struct Log;

impl Log {
    fn new() -> Self {
        Self
    }
}

impl Middleware for Log {
    fn on_request(&self, req: &mut HttpRequest, ctx: &Context) -> bool {
        println!("request method: {:?}", req.method);
        true
    }

    fn on_response(&self, req: &HttpRequest, res: &mut HttpResponse, ctx: &Context) {
        println!("Response body {:?}", res.body);
    }
}

fn main() {
    use milim_web::request::Method::*;
    let mut app = server();

    app.route_use("/", Get, [Log], user);

    app.route("/:name", Get, |req, res, ctx| {
        res.body(&format!(
            "O valor de name e: {}",
            req.get_param("name").unwrap_or("".to_string())
        ));
    });

    app.route("/:id/:name", Get, |req, res, ctx| {
        res.body(&format!(
            "Id: {}, Name: {}",
            req.get_param("id").unwrap_or("".to_string()),
            req.get_param("name").unwrap_or("".to_string())
        ));
    });

    app.listen("127.0.0.1:3000")
}
