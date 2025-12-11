use std::{collections::HashMap, io::Result};

use crate::router::IntoBody;

#[derive(Clone, Debug, PartialEq)]
pub struct HttpResponse {
    pub version: String,
    pub status_code: String,
    pub status_text: String,
    pub headers: Option<HashMap<String, String>>,
    pub body: Option<String>,
}

impl Default for HttpResponse {
    fn default() -> Self {
        Self {
            version: "HTTP/1.1".to_string(),
            status_code: "200".to_string(),
            status_text: "OK".to_string(),
            headers: None,
            body: None,
        }
    }
}

impl HttpResponse {
    pub fn new(
        status_code: &str,
        headers: Option<HashMap<String, String>>,
        body: Option<String>,
    ) -> Self {
        let mut response: HttpResponse = HttpResponse::default();
        if status_code != "200" {
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
        response.status_text = match response.status_code.as_str() {
            "200" => "OK".to_string(),
            "400" => "Bad Request".to_string(),
            "404" => "Not Found".to_string(),
            "405" => "Method not Allowed".to_string(),
            "500" => "Internal Server Error".to_string(),
            _ => "Not Found".to_string(),
        };
        response.body = body;
        response
    }

    pub fn get_version(&self) -> &str {
        &self.version
    }
    pub fn get_status_code(&self) -> &str {
        &self.status_code
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
        match &self.body {
            Some(b) => b.as_str(),
            None => "",
        }
    }
    pub fn body<I: IntoBody>(&mut self, body: I) -> Self {
        self.body = Some(body.into_body());
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
