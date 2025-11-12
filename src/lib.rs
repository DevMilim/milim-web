//! # Biblioteca web criada totalmente em std
//! # Exemplo de uso:
//! `````` rust
//! use milim_web::server;
//! use milim_web::request::Method::*;
//!
//! fn main() {
//!    let mut app = server();
//!
//!    app.route("/", |req, res, ctx| {
//!        let query = req.get_query("name").unwrap_or("".to_string());
//!        res.body(&format!("Valor da query name e: {}", query))
//!    });
//!
//!    app.route("/id/:name", |req, res, ctx| {
//!        res.body(&format!(
//!            "O valor de name e: {}",
//!            req.get_param("name").unwrap_or("".to_string())
//!        ));
//!    });
//!
//!    app.listen("127.0.0.1:3000")
//!}
//!
//!
//! ```
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
