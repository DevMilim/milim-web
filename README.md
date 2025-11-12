# Uma biblioteca de servidor http escrita em rust.

# Exemplo de uso

``` rust
use milim_web::request::Method::*;
use milim_web::server;

fn main() {
    // Cria uma instancia de App
    let mut app = server();

    app.route("/", Get, |req, res, ctx| {
        // Obtem a query name se definida
        let query = req.get_query("name").unwrap_or("".to_string());
        res.body(&format!("Valor da query name e: {}", query));
    });
    // Cria uma rota dinamica com o parametro name
    app.route("/id/:name", Get, |req, res, ctx| {
        res.body(&format!(
            "O valor de name e: {}",
            req.get_param("name").unwrap_or("".to_string())
        ));
    });

    // Inicia o servidor Http na porta 3000
    app.listen("127.0.0.1:3000")
}

```

# Exemplo de Middleware
``` rust
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

```

# Criando rota que usa esse Middleware

``` rust
app.route_use("/", Get, [Log], |req, res, ctx| {
    res.body("Hello World!!");
});
```