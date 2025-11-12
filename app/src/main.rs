use std::io::Result;

use milim_web::{
    context::Context,
    mx,
    request::HttpRequest,
    response::HttpResponse,
    router::{Middleware, MwFlow},
    server,
};

fn user(req: &HttpRequest, res: &mut HttpResponse, ctx: &Context) {
    println!("2: Passou");
    res.body("funcion");
}

pub struct Log {}

impl Middleware for Log {
    fn on_request(&self, req: &mut HttpRequest, ctx: &Context) -> MwFlow {
        println!("1: request method: {:?}", req.method);

        MwFlow::Continue
    }

    fn on_response(&self, req: &HttpRequest, res: &mut HttpResponse, ctx: &Context) {
        println!("3: Response body {:?}", res.body);
    }
}
fn main() -> Result<()> {
    use milim_web::request::Method::*;
    let mut app = server();

    app.route_use("/", Get, mx!(Log {}), user);

    app.route("/get/:name", Get, |req, res, ctx| {
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
