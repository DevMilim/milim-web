use crate::{response::HttpResponse, status::StatusCode};

pub trait Responder: Send {
    fn into_response(self) -> HttpResponse;
}

impl Responder for &str {
    fn into_response(self) -> HttpResponse {
        HttpResponse::new(StatusCode::Ok, None, self)
    }
}
impl Responder for String {
    fn into_response(self) -> HttpResponse {
        HttpResponse::new(StatusCode::Ok, None, self)
    }
}
impl Responder for HttpResponse {
    fn into_response(self) -> HttpResponse {
        self
    }
}
