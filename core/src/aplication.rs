use std::{
    collections::HashMap,
    io::{Read, Result, Write},
    net::{TcpListener, TcpStream},
    sync::Arc,
};

use crate::{
    context::Context,
    request::{HttpRequest, Resource},
    responses::Responder,
};

type Handler = Box<
    dyn Fn(&HttpRequest, &Context) -> Box<dyn Responder<'static> + Send> + Send + Sync + 'static,
>;

pub struct App {
    routes: Vec<(String, Handler)>,
    context: Context,
}

impl App {
    pub fn new() -> Self {
        Self {
            routes: Vec::new(),
            context: Context {},
        }
    }
    pub fn get<
        F: Fn(&HttpRequest) -> R + Send + Sync + 'static,
        R: Responder<'static> + Send + 'static,
    >(
        &mut self,
        path: &str,
        handler: F,
    ) {
        let wrapper = Box::new(move |req: &HttpRequest, ctx: &Context| {
            Box::new(handler(req)) as Box<dyn Responder + Send>
        });
        self.routes.push((path.to_string(), wrapper));
    }

    pub fn listen(self, adress: &str) {
        let listener = TcpListener::bind(adress).expect("Erro ao iniciar servidor");
        let routes = Arc::new(self.routes);

        for stream in listener.incoming() {
            match stream {
                Ok(mut stream) => {
                    let _ = handle_connection(&routes, &mut stream, &self.context);
                }
                Err(e) => eprintln!("Erro"),
            }
        }
    }
}

fn handle_connection(
    routes: &[(String, Handler)],
    stream: &mut TcpStream,
    ctx: &Context,
) -> Result<()> {
    let mut buf = [0u8; 8192];
    let n = stream.read(&mut buf).expect("Erro ao ler stream");
    if n == 0 {
        return Ok(());
    }

    let raw = String::from_utf8_lossy(&buf[..n]).to_string();
    println!("Raw request: \n{}", raw);

    let mut req = HttpRequest::from(raw);
    println!("{:#?}", req);
    let path = match &req.resource {
        Resource::Path(p) => p.clone(),
    };

    for (pattern, handler) in routes {
        if let Some(params) = match_route(pattern, &path) {
            req.params = Some(params.0);
            req.queryes = Some(params.1);

            let res_string: String = handler(&req, ctx).response().into();
            stream
                .write_all(res_string.as_bytes())
                .expect("Erro ao escrever buffer de resposta");
            return Ok(());
        }
    }
    Ok(())
}

fn split_path_query(s: &str) -> (&str, Option<&str>) {
    if let Some(pos) = s.find("?") {
        (&s[..pos], Some(&s[pos + 1..]))
    } else {
        (&s, None)
    }
}
fn parse_query(q: Option<&str>) -> HashMap<String, String> {
    let mut map = HashMap::new();
    if let Some(qs) = q {
        for pair in qs.split("&") {
            if pair.is_empty() {
                continue;
            }
            let mut it = pair.splitn(2, "=");
            let key = it.next().unwrap_or("");
            let val = it.next().unwrap_or("");
            map.insert(key.to_string(), val.to_string());
        }
    }
    map
}

fn percent_decode(s: &str) -> String {
    String::new()
}

fn match_route(
    pattern: &str,
    path: &str,
) -> Option<(HashMap<String, String>, HashMap<String, String>)> {
    let (path_only, query_opt) = split_path_query(path);
    let pat_parts: Vec<&str> = pattern.trim_matches('/').split('/').collect();
    let path_parts: Vec<&str> = path_only.trim_matches('/').split('/').collect();
    if pat_parts.len() != path_parts.len() {
        return None;
    }
    let mut params = HashMap::new();
    for (pp, rp) in pat_parts.iter().zip(path_parts.iter()) {
        if pp.starts_with(":") {
            let name = pp.trim_start_matches(":").to_string();
            println!(">> Param: {}\n>> Value: {}", name, rp);
            params.insert(name, rp.to_string());
        } else if pp != rp {
            return None;
        }
    }

    let query_params = parse_query(query_opt);

    Some((params, query_params))
}
