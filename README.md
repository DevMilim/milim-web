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

```

Execute:
```shell
cargo run
```


Visite ```http://localhost:3000/hello``` e vera o resultado ```Hello World!!```
