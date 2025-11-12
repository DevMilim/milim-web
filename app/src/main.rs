use milim_web::{
    context::Context, mx, request::HttpRequest, response::HttpResponse, router::Middleware, server,
};

fn user(req: &HttpRequest, res: &mut HttpResponse, ctx: &Context) {
    println!("2: Passou");
    res.body("funcion");
}

pub struct Log {}

impl Middleware for Log {
    fn on_request(&self, req: &mut HttpRequest, ctx: &Context) -> bool {
        println!("1: request method: {:?}", req.method);

        true
    }

    fn on_response(&self, req: &HttpRequest, res: &mut HttpResponse, ctx: &Context) {
        println!("3: Response body {:?}", res.body);
    }
}
pub struct Log2;

impl Middleware for Log2 {
    fn on_request(&self, req: &mut HttpRequest, ctx: &Context) -> bool {
        println!("1-2: request method: {:?}", req.method);
        false
    }

    fn on_response(&self, req: &HttpRequest, res: &mut HttpResponse, ctx: &Context) {
        println!("3-2: Response body {:?}", res.body);
    }
}

fn main() {
    use milim_web::request::Method::*;
    let mut app = server();

    app.route_use("/", Get, mx!(Log {}, Log2), user);

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
