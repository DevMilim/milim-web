use std::{
    collections::HashMap,
    io::{Read, Result, Write},
    net::{TcpListener, TcpStream},
    sync::Arc,
    time::Duration,
};

use crate::{
    config::Config,
    context::Context,
    request::{HttpRequest, Method, Resource},
    response::HttpResponse,
    router::{Middleware, Router},
};

pub struct App {
    routes: Vec<Router>,
    context: Context,
    config: Config,
}

impl App {
    pub fn new() -> Self {
        Self {
            routes: Vec::new(),
            context: Context {},
            config: Config::new(),
        }
    }
    /// Adiciona rota que usa middlewares
    pub fn route_use<F, I, M>(&mut self, path: &str, method: Method, middlewares: I, handler: F)
    where
        F: Fn(&HttpRequest, &mut HttpResponse, &Context) + Send + Sync + 'static,
        I: IntoIterator<Item = M>,
        M: Middleware,
    {
        let wrapper = Arc::new(handler);

        let wrapper_m: Vec<Arc<dyn Middleware>> = middlewares
            .into_iter()
            .map(|m| Arc::new(m) as Arc<dyn Middleware>)
            .collect();

        self.routes
            .push(Router::new(path, wrapper, method, wrapper_m));
    }
    pub fn route<F>(&mut self, path: &str, method: Method, handler: F)
    where
        F: Fn(&HttpRequest, &mut HttpResponse, &Context) + Send + Sync + 'static,
    {
        let wrapper = Arc::new(handler);

        self.routes
            .push(Router::new(path, wrapper, method, Vec::new()));
    }

    pub fn listen(self, adress: &str) {
        println!(" > Max body size: {}KB", self.config.max_body_kb);
        println!(" > Keep alive: {}s", self.config.keep_alive_s);
        println!(" > Max headers: {}", self.config.max_headers);
        let listener = TcpListener::bind(adress).expect("Erro ao iniciar servidor");
        let routes = Arc::new(self.routes);
        let ctx = Arc::new(self.context);

        for stream in listener.incoming() {
            match stream {
                Ok(mut stream) => {
                    stream
                        .set_read_timeout(Some(Duration::from_secs(
                            self.config.read_timeout_s.into(),
                        )))
                        .ok();
                    stream
                        .set_write_timeout(Some(Duration::from_secs(
                            self.config.read_timeout_s.into(),
                        )))
                        .ok();
                    let _ = handle_connection(&*routes, &mut stream, &*ctx, &self.config);
                }
                Err(e) => eprintln!("Erro ao aceitar a conexão? {:?}", e),
            }
        }
    }
}

fn handle_connection(
    routes: &Vec<Router>,
    stream: &mut TcpStream,
    ctx: &Context,
    config: &Config,
) -> Result<()> {
    let mut buf = vec![0u8; Config::get_kb_value(config.max_body_kb)];
    let n = stream.read(&mut buf).expect("Erro ao ler stream");
    if n == 0 {
        return Ok(());
    }

    let raw = String::from_utf8_lossy(&buf[..n]).to_string();

    let mut req = HttpRequest::from(raw);

    let path = match &req.resource {
        Resource::Path(p) => p.clone(),
    };

    let mut found_path = false;
    for route in routes {
        if let Some(params) = match_route(&route.pattern, &path) {
            found_path = true;
            if route.method != req.method {
                continue;
            }
            req.params = Some(params.0);
            req.queryes = Some(params.1);

            let mut res = HttpResponse::new("200", None, Some("".to_string()));

            // Executando middlewares
            let mut executed = Vec::new();
            let mut continue_flow = true;

            for mw in route.route_middlewares.iter().rev() {
                continue_flow = mw.on_request(&mut req, ctx);
                executed.push(Arc::clone(mw));
                if !continue_flow {
                    break;
                }
            }
            if continue_flow {
                (route.handler)(&req, &mut res, ctx);
            }

            while let Some(mw) = executed.pop() {
                mw.on_response(&req, &mut res, ctx);
            }

            let res_string: String = res.into();

            stream
                .write_all(res_string.as_bytes())
                .expect("Erro ao escrever buffer de resposta");
            return Ok(());
        }
    }
    if !found_path {
        let res: String = HttpResponse::new("404", None, Some("Not Found".to_string())).into();
        let _ = stream.write_all(res.as_bytes());
        return Ok(());
    }
    let res: String = HttpResponse::new("405", None, Some("".to_string())).into();
    let _ = stream.write_all(res.as_bytes());
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

fn _percent_decode(_s: &str) -> String {
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
