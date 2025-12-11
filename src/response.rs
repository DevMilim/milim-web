use std::{collections::HashMap, io::Result};

use crate::{router::IntoBody, status::StatusCode};

#[derive(Clone, Debug, PartialEq)]
pub struct HttpResponse {
    pub version: String,
    pub status_code: StatusCode,
    pub status_text: String,
    pub headers: Option<HashMap<String, String>>,
    pub body: String,
}

impl Default for HttpResponse {
    fn default() -> Self {
        Self {
            version: "HTTP/1.1".to_string(),
            status_code: StatusCode::Ok,
            status_text: "OK".to_string(),
            headers: None,
            body: "".to_string(),
        }
    }
}

impl HttpResponse {
    pub fn new(
        status_code: StatusCode,
        headers: Option<HashMap<String, String>>,
        body: impl IntoBody,
    ) -> Self {
        let mut response: HttpResponse = HttpResponse::default();
        if status_code != StatusCode::Ok {
            response.status_code = status_code.into();
        }
        response.headers = match &headers {
            Some(_) => headers,
            None => {
                let mut h = HashMap::new();
                h.insert("Content-Type".to_string(), "text/html".to_string());
                Some(h)
            }
        };
        response.status_text = response.get_status_code().to_string();
        response.body = body.into_body();
        response
    }

    pub fn get_version(&self) -> &str {
        &self.version
    }
    pub fn get_status_code(&self) -> &str {
        match self.status_code {
            StatusCode::Ok => "200",
            StatusCode::NotAutorized => "Not Autorized",
            StatusCode::BadRequest => "Bad Request",
            StatusCode::NotFound => "Not Found",
            StatusCode::MethodNotAllowed => "Method Not Allowed",
            StatusCode::InternalServerError => "Internal Server Error",
        }
    }
    pub fn get_status_text(&self) -> &str {
        &self.status_text
    }
    pub fn get_headers(&self) -> String {
        let map: HashMap<String, String> = self.headers.clone().expect("Erro ao obter headers");
        let mut header_string: String = "".into();
        for (k, v) in map.iter() {
            header_string = format!("{}{}:{}\r\n", header_string, k, v);
        }
        header_string
    }
    pub fn get_body(&self) -> &str {
        &self.body
    }
    pub fn body<I: IntoBody>(&mut self, body: I) -> Self {
        self.body = body.into_body();
        self.clone()
    }
    pub fn add_header(&mut self, key: &str, value: &str) -> Result<()> {
        let mut h = match self.headers.clone() {
            Some(headers) => headers,
            None => HashMap::new(),
        };
        h.insert(key.to_string(), value.to_string());
        self.headers = Some(h);
        Ok(())
    }
    pub fn remove_header(&mut self, key: &str) -> Result<()> {
        let mut h = match self.headers.clone() {
            Some(headers) => headers,
            None => HashMap::new(),
        };
        h.remove(key);
        self.headers = Some(h);
        Ok(())
    }
}

impl From<HttpResponse> for String {
    fn from(value: HttpResponse) -> Self {
        let res = value.clone();
        format!(
            "{} {} {}\r\n{}Content-Length: {}\r\n\r\n{}",
            &res.get_version(),
            &res.get_status_code(),
            &res.get_status_text(),
            &res.get_headers(),
            &res.get_body().len(),
            &res.get_body()
        )
    }
}
