use crate::{response::HttpResponse, status::StatusCode};

pub trait Responder: Send {
    fn into_response(self: Box<Self>) -> HttpResponse;
}

impl Responder for &str {
    fn into_response(self: Box<Self>) -> HttpResponse {
        HttpResponse::new(StatusCode::Ok, None, *self)
    }
}
impl Responder for String {
    fn into_response(self: Box<Self>) -> HttpResponse {
        HttpResponse::new(StatusCode::Ok, None, *self)
    }
}
impl Responder for HttpResponse {
    fn into_response(self: Box<Self>) -> HttpResponse {
        *self
    }
}
impl Responder for Box<dyn Responder + Send> {
    fn into_response(self: Box<Self>) -> HttpResponse {
        (*self).into_response()
    }
}
