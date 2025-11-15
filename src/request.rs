use std::collections::HashMap;

// Enum que representa o metodo da requisição http
#[derive(Debug, PartialEq, Clone)]
pub enum Method {
    Get,
    Post,
    Uninitialized,
}

/// Implementa a trait From<&str> para poder converter &str para o enum Method
impl From<&str> for Method {
    fn from(value: &str) -> Self {
        match value {
            "GET" | "get" => Method::Get,
            "POST" | "post" => Method::Post,
            _ => Method::Uninitialized,
        }
    }
}

/// Enum representando a versão no header da requisição http
#[derive(Debug, PartialEq)]
pub enum Version {
    V1_1,
    V2_0,
    Uninitialized,
}

/// implementa a trait From<&str> para Version para converter &str para o enum Version
impl From<&str> for Version {
    fn from(value: &str) -> Self {
        match value {
            "HTTP/1.1" => Version::V1_1,
            "HTTP/2.0" => Version::V2_0,
            _ => Version::Uninitialized,
        }
    }
}

/// Enum que representa o caminho no header da requisição http
#[derive(Debug, PartialEq)]
pub enum Resource {
    Path(String),
}

/// Struct que representa a requisição http
#[derive(Debug)]
pub struct HttpRequest {
    pub method: Method,
    pub version: Version,
    pub resource: Resource,
    pub headers: HashMap<String, String>,
    pub body: String,
    pub(crate) params: Option<HashMap<String, String>>,
    pub(crate) queryes: Option<HashMap<String, String>>,
}
impl HttpRequest {
    pub fn get_param(&self, key: &str) -> Option<String> {
        self.params.clone()?.get(key).cloned()
    }
    pub fn get_query(&self, key: &str) -> Option<String> {
        self.queryes.clone()?.get(key).cloned()
    }
}
/// Implementa From<String> para
impl From<String> for HttpRequest {
    fn from(req: String) -> Self {
        // Variaveis que armazenam os campos da requisição
        let mut parsed_method = Method::Uninitialized;
        let mut parsed_version = Version::V1_1;
        let mut parsed_resource = Resource::Path("".to_string());
        let mut parsed_headers = HashMap::new();

        // Separa o head do body da requisição
        let parts: Vec<&str> = req.splitn(2, "\r\n\r\n").collect();
        let head = parts.get(0).unwrap_or(&"");
        let body_part = parts.get(1).unwrap_or(&"");

        for line in head.lines() {
            if line.contains("HTTP") {
                let (method, resource, version) = process_req_line(line);
                parsed_method = method;
                parsed_version = version;
                parsed_resource = resource;
            } else if line.contains(":") {
                let (key, value) = process_header_line(line);
                parsed_headers.insert(key, value);
            }
        }

        HttpRequest {
            method: parsed_method,
            version: parsed_version,
            resource: parsed_resource,
            headers: parsed_headers,
            body: body_part.to_string(),
            params: None,
            queryes: None,
        }
    }
}

/// Processa a primeira linha da requisição
fn process_req_line(s: &str) -> (Method, Resource, Version) {
    let mut words = s.split_whitespace();
    let method = words.next().unwrap();
    let resource = words.next().unwrap();
    let version = words.next().unwrap();
    (
        method.into(),
        Resource::Path(resource.into()),
        version.into(),
    )
}

/// Processa os headers da requisição
fn process_header_line(s: &str) -> (String, String) {
    let mut header_items = s.split(":");
    let mut key = String::from("");
    let mut value = String::from("");
    if let Some(k) = header_items.next() {
        key = k.to_string()
    }
    if let Some(v) = header_items.next() {
        value = v.trim().to_string()
    }
    (key, value)
}
