/*!
# Uma biblioteca web minimalista escrita em rust.

## Sobre
Uma biblioteca web leve e modular desenvolvida em **Rust**, projetada para facilitar a criação de APIs rapidas e seguras,
com sintaxe simples, similar ao express.js

# Exemplo de uso

## Dependencia
```shell
cargo add milim-web --git https://github.com/DevMilim/milim-web
```
ou

``` toml
[dependencies]
milim-web = { version = "0.1.0", git = "https://github.com/DevMilim/milim-web" }

```

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
    app.route("/:name", Get, |req, res, ctx| {
        res.body(&format!(
            "O valor de name e: {}",
            req.get_param("name").unwrap_or("".to_string())
        ));
    });

    // Inicia o servidor Http na porta 3000
    app.listen("127.0.0.1:3000")
}

```
Visite ```localhost:3000/username``` e vera o resultado ```O valor de name e: username```

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

---
*/
use crate::aplication::App;
pub mod aplication;
pub mod config;
pub mod context;
pub mod error;
pub mod prelude;
pub mod request;
pub mod response;
pub mod router;
pub mod tests;

pub fn server() -> App {
    App::new()
}
