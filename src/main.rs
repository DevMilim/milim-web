use core::server;

fn main() {
    let mut app = server();

    app.get("/", |req| {
        let query = req.get_query("name").unwrap_or("".to_string());
        format!("Valor da query name e: {}", query)
    });

    app.get("/id/:name", |req| {
        format!(
            "O valor de name e: {}",
            req.get_param("name").unwrap_or("".to_string())
        )
    });

    app.get("/:id/:name", |req| "id e name obtido");

    app.listen("127.0.0.1:3000")
}
