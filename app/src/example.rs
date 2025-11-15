use std::io::Result;

use milim_web::{
    context::Context,
    request::HttpRequest,
    response::HttpResponse,
    router::{Middleware, MwFlow},
    server,
};

// Função Handler para a rota "/"
fn hello(req: &HttpRequest, res: &mut HttpResponse, ctx: &Context) {
    res.body("Hello");
}

// Definição de Middleware
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

    // Registrando Middleware global
    app.global_use(Log {});

    // Defininado uma rota Get em "/" e usando a dunção de handler
    app.route(Get, "/").handler(hello);

    // Definindo uma rota Get dinamica
    app.route(Get, "/get/:name").handler(
        |req: &HttpRequest, res: &mut HttpResponse, ctx: &Context| {
            // Obtendo o parametro name
            let id = req.get_param("name").unwrap_or("".to_string());
            res.body(&format!("O valor de name e: {}", id));
        },
    );

    app.listen("127.0.0.1:3000")
}
