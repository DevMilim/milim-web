use crate::responses::HttpResponse;

pub trait Responder<'a> {
    fn response(&self) -> HttpResponse<'a>;
}

impl<'a> Responder<'a> for HttpResponse<'a> {
    fn response(&self) -> HttpResponse<'a> {
        self.clone()
    }
}

impl<'a> Responder<'a> for &'a str {
    fn response(&self) -> HttpResponse<'a>
    where
        Self: Sized,
    {
        HttpResponse::new("200", None, Some(self.to_string()))
    }
}

impl<'a> Responder<'a> for String {
    fn response(&self) -> HttpResponse<'a>
    where
        Self: Sized,
    {
        HttpResponse::new("200", None, Some(self.to_string()))
    }
}
impl<'a> Responder<'a> for i32 {
    fn response(&self) -> HttpResponse<'a>
    where
        Self: Sized,
    {
        HttpResponse::new("200", None, Some(self.to_string()))
    }
}
impl<'a> Responder<'a> for i64 {
    fn response(&self) -> HttpResponse<'a>
    where
        Self: Sized,
    {
        HttpResponse::new("200", None, Some(self.to_string()))
    }
}
impl<'a> Responder<'a> for i8 {
    fn response(&self) -> HttpResponse<'a>
    where
        Self: Sized,
    {
        HttpResponse::new("200", None, Some(self.to_string()))
    }
}
impl<'a> Responder<'a> for u32 {
    fn response(&self) -> HttpResponse<'a>
    where
        Self: Sized,
    {
        HttpResponse::new("200", None, Some(self.to_string()))
    }
}
