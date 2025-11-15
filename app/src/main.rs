use std::{
    io::Result,
    sync::{Mutex, RwLock},
};
mod example;

use milim_web::{
    context::Context,
    request::HttpRequest,
    response::HttpResponse,
    router::{Middleware, MwFlow},
    server,
};

fn user(req: &HttpRequest, res: &mut HttpResponse, ctx: &Context) {
    let state = ctx.get_state::<Mutex<u32>>();
    match state {
        Some(value) => {
            let mut v = value.lock().unwrap();
            res.body(&format!("Value: {:?}", v));
            *v += 1;
        }
        None => todo!(),
    }
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

    app.global_use(Log {});
    let state: Mutex<u32> = Mutex::new(0);
    app.manage(state);

    app.route(Get, "/").handler(user);

    app.route(Get, "/get/:name").handler(
        |req: &HttpRequest, res: &mut HttpResponse, ctx: &Context| {
            res.body(&format!(
                "O valor de name e: {}",
                req.get_param("name").unwrap_or("".to_string())
            ));
        },
    );

    app.route(Get, "/:id/:name").handler(
        |req: &HttpRequest, res: &mut HttpResponse, ctx: &Context| {
            res.body(&format!(
                "Id: {}, Name: {}",
                req.get_param("id").unwrap_or("".to_string()),
                req.get_param("name").unwrap_or("".to_string())
            ));
        },
    );

    app.listen("127.0.0.1:3000")
}
