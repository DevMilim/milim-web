use std::{collections::HashMap, io::Result, ops::DerefMut, sync::Arc};

use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::TcpListener,
};

use crate::{
    config::Config,
    context::Context,
    fairing::{Fairing, IntoFairing},
    guard::Outcome,
    request::{HttpRequest, HttpRequestData, Method, Resource},
    response::HttpResponse,
    router::{RouteBuilder, Router},
    status::StatusCode,
};

pub struct App {
    routes: Vec<Router>,
    context: Context,
    config: Config,
    fairings: Vec<Arc<dyn Fairing>>,
}

impl App {
    pub fn new() -> Self {
        Self {
            routes: Vec::new(),
            context: Context {
                map: HashMap::new(),
            },
            config: Config::new(),
            fairings: Vec::new(),
        }
    }
    /// # Usado para adicionar um middleware global ele sera executado antes dos de rota
    pub fn fairing<M: IntoFairing>(&mut self, middleware: M) {
        self.fairings.push(middleware.into_fairing());
    }
    /// Adiciona uma rota
    pub fn route<'a>(&'a mut self, method: Method, path: &str) -> RouteBuilder<'a> {
        RouteBuilder {
            app: self,
            pattern: path.to_owned(),
            method,
            guards: Vec::new(),
        }
    }
    pub fn manage<T: Send + Sync + 'static>(&mut self, state: T) {
        self.context.state(state)
    }
    pub(crate) fn add_route(&mut self, route: Router) {
        self.routes.push(route);
    }
    /// Inicia um servidor http sync
    pub async fn listen(&mut self, adress: &str) -> Result<()> {
        println!(" > Max body size: {}KB", self.config.max_body_kb);
        println!(" > Keep alive: {}s", self.config.keep_alive_s);
        println!(" > Max headers: {}", self.config.max_headers);
        let listener = TcpListener::bind(adress).await?;

        let App {
            routes,
            context,
            config,
            fairings,
        } = self;
        for fairings in fairings.iter() {
            fairings.on_ready(context).await;
        }
        loop {
            let (mut socket, _) = listener.accept().await?;

            let mut buf = vec![0u8; Config::get_kb_value(config.max_body_kb)];

            match socket.read(&mut buf).await {
                Ok(n) if n > 0 => {
                    let raw = String::from_utf8_lossy(&buf[..n]).to_string();

                    let req_data = HttpRequestData::from(raw);
                    let mut req = HttpRequest::new(req_data);

                    let path = match &req.raw.resource {
                        Resource::Path(p) => p.clone(),
                    };

                    let mut handled = false;
                    let mut method_mismatch = false;
                    for route in routes.iter() {
                        if let Some((params, queryes)) = match_route(&route.pattern, &path) {
                            if route.method != req.raw.method {
                                method_mismatch = true;
                                continue;
                            }
                            req.raw.params = Some(params);
                            req.raw.queryes = Some(queryes);

                            let mut res = HttpResponse::new(StatusCode::Ok, None, "");
                            let mut executed = Vec::new();

                            // Executando os Fairings e registra em executed para executar on_response
                            for fairings in fairings.iter() {
                                fairings.on_request(&mut req, context).await;
                                executed.push(Arc::clone(fairings));
                            }
                            let mut outcome = Outcome::Success;
                            // Executa os guards
                            for guard in route.guards.iter() {
                                outcome = guard.from_request(&req, context).await;
                                if outcome != Outcome::Success {
                                    break;
                                }
                            }
                            if let Outcome::Failure(response) = outcome {
                                res = response;
                            } else {
                                (route.handler)(&req, context);
                            }
                            for f in executed.iter() {
                                f.on_response(&req, &mut res, context).await;
                            }
                            let res_string: String = res.into();

                            socket.write_all(res_string.as_bytes()).await?;

                            handled = true;
                            break;
                        }
                    }
                    if handled {
                        continue;
                    }
                    if method_mismatch {
                        let res: String =
                            HttpResponse::new(StatusCode::MethodNotAllowed, None, "".to_string())
                                .into();
                        let _ = socket.write_all(res.as_bytes()).await?;
                        continue;
                    } else {
                        let res: String =
                            HttpResponse::new(StatusCode::NotFound, None, "Not Found").into();
                        let _ = socket.write_all(res.as_bytes()).await?;
                        continue;
                    }
                }
                Ok(_) => {
                    println!("Not found")
                }
                Err(_) => {
                    println!("Erro")
                }
            };
        }
    }
}

/// Separa o path da query e retorna
fn split_path_query(s: &str) -> (&str, Option<&str>) {
    if let Some(pos) = s.find("?") {
        (&s[..pos], Some(&s[pos + 1..]))
    } else {
        (&s, None)
    }
}

/// Faz o parse da query obtendo a chave e valor
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

/// Ainda não implementado
fn _percent_decode(_s: &str) -> String {
    String::new()
}

/// Obtem as rotas e os parametros da requisição
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
