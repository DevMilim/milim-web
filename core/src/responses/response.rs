use std::{
    collections::HashMap,
    io::{Result, Write},
};

#[derive(Clone, Debug, PartialEq)]
pub struct HttpResponse<'a> {
    pub(crate) version: &'a str,
    pub(crate) status_code: &'a str,
    pub(crate) status_text: &'a str,
    pub(crate) headers: Option<HashMap<&'a str, &'a str>>,
    pub(crate) body: Option<String>,
}

impl<'a> Default for HttpResponse<'a> {
    fn default() -> Self {
        Self {
            version: "HTTP/1.1",
            status_code: "200",
            status_text: "OK",
            headers: None,
            body: None,
        }
    }
}

impl<'a> HttpResponse<'a> {
    pub fn new(
        status_code: &'a str,
        headers: Option<HashMap<&'a str, &'a str>>,
        body: Option<String>,
    ) -> Self {
        let mut response: HttpResponse<'a> = HttpResponse::default();
        if status_code != "200" {
            response.status_code = status_code.into();
        }
        response.headers = match &headers {
            Some(_) => headers,
            None => {
                let mut h = HashMap::new();
                h.insert("Content-Type", "text/html");
                Some(h)
            }
        };
        response.status_text = match response.status_code {
            "200" => "OK",
            "400" => "Bad Request",
            "404" => "Not Found",
            "500" => "Internal Server Error",
            _ => "Not Found",
        };
        response.body = body;
        response
    }

    pub fn send_response(&self, write_stream: &mut impl Write) -> Result<()> {
        let res = self.clone();
        let response_string: String = String::from(res);
        let _ = write!(write_stream, "{}", response_string);
        Ok(())
    }
    pub fn get_version(&self) -> &str {
        self.version
    }
    pub fn get_status_code(&self) -> &str {
        self.status_code
    }
    pub fn get_status_text(&self) -> &str {
        self.status_text
    }
    pub fn get_headers(&self) -> String {
        let map: HashMap<&str, &str> = self.headers.clone().expect("Erro ao obter headers");
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
    pub fn body(&mut self, body: &str) -> Self {
        self.body = Some(body.to_string());
        self.clone()
    }
    pub fn ok() -> Self {
        HttpResponse::new("200", None, Some("".to_string()))
    }
}

impl<'a> From<HttpResponse<'a>> for String {
    fn from(value: HttpResponse<'a>) -> Self {
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
