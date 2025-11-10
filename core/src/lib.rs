use crate::aplication::App;

pub mod aplication;
pub mod config;
pub mod context;
pub mod error;
pub mod prelude;
pub mod request;
pub mod responses;
pub mod router;
pub mod tests;

pub fn server() -> App {
    App::new()
}
