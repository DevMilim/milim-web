# Uma biblioteca de servidor http escrita em rust.

# Exemplo de uso

``` rust
use milim_web::request::Method::*;
use milim_web::server;

fn main() {
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