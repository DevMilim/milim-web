use std::{collections::HashMap, io::Result, sync::Arc};

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
    context: Arc<Context>, // keep Arc<Context>
    config: Config,
    fairings: Vec<Arc<dyn Fairing>>,
}

impl App {
    pub fn new() -> Self {
        Self {
            routes: Vec::new(),
            context: Arc::new(Context {
                map: HashMap::new(),
            }),
            config: Config::new(),
            fairings: Vec::new(),
        }
    }

    pub fn fairing<M: IntoFairing>(&mut self, middleware: M) {
        self.fairings.push(middleware.into_fairing());
    }

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

    /// Inicia um servidor http async
    pub async fn listen(&mut self, adress: &str) -> Result<()> {
        println!(" > Max body size: {}KB", self.config.max_body_kb);
        println!(" > Keep alive: {}s", self.config.keep_alive_s);
        println!(" > Max headers: {}", self.config.max_headers);
        let listener = TcpListener::bind(adress).await?;

        // não movemos `self` — apenas clonamos os Arcs e pegamos referências onde preciso
        let context = Arc::clone(&self.context);
        let config = &self.config;
        let fairings = &self.fairings;
        let routes = &self.routes;

        // ready hooks (passando Arc<Context> para fairings, se a API dos fairings aceitar Arc)
        for f in fairings.iter() {
            // se on_ready aceita &Context, use f.on_ready(&*context).await;
            // aqui assumo que on_ready aceita Arc<Context> como antes.
            f.on_ready(Arc::clone(&context)).await;
        }

        loop {
            let (mut socket, _) = listener.accept().await?;

            let mut buf = vec![0u8; Config::get_kb_value(config.max_body_kb)];

            match socket.read(&mut buf).await {
                Ok(n) if n > 0 => {
                    let raw = String::from_utf8_lossy(&buf[..n]).to_string();

                    let req_data = HttpRequestData::from(raw);
                    // cria req mutável (ainda não em Arc)
                    let mut req = HttpRequest::new(req_data);

                    let path = match &req.raw.resource {
                        Resource::Path(p) => p.clone(),
                    };

                    let mut handled = false;
                    let mut method_mismatch = false;

                    'routes: for route in routes.iter() {
                        if let Some((params, queryes)) = match_route(&route.pattern, &path) {
                            if route.method != req.raw.method {
                                method_mismatch = true;
                                continue 'routes;
                            }

                            req.raw.params = Some(params);
                            req.raw.queryes = Some(queryes);

                            // resposta default (pode ser sobrescrita pelo handler/responder)
                            // você pode manter essa inicialização ou derivar do Responder retornado
                            let mut res = HttpResponse::new(StatusCode::Ok, None, "");

                            // Executando os Fairings e registra em executed para executar on_response
                            let mut executed = Vec::new();
                            for f in fairings.iter() {
                                // fairing.on_request provavelmente aceita (&mut HttpRequest, &Context)
                                // passamos &*context se for &Context, ou Arc::clone(&context) se aceitar Arc
                                // aqui uso &*context para minimizar mudanças:
                                f.on_request(&mut req, &*context).await;
                                executed.push(Arc::clone(f));
                            }

                            // Executa os guards (supondo que from_request aceita &HttpRequest, &Context)
                            let mut outcome = Outcome::Success;
                            for guard in route.guards.iter() {
                                outcome = guard.from_request(&req, &*context).await;
                                if outcome != Outcome::Success {
                                    break;
                                }
                            }

                            if let Outcome::Failure(response) = outcome {
                                res = response;
                            } else {
                                // --- CALL HANDLER: criar Arc<HttpRequest> e passar Arc<Context> ---
                                // move req para um Arc e use Arc::clone(&context)
                                let req_arc = Arc::new(req);
                                // call handler: assume handler signature -> fn(Arc<HttpRequest>, Arc<Context>) -> BoxFuture<...>
                                let responder_box =
                                    (route.handler)(Arc::clone(&req_arc), Arc::clone(&context))
                                        .await;

                                // --- converter Box<dyn Responder> para HttpResponse ---
                                // aqui depende da tua trait Responder. Exemplos:
                                //
                                // 1) se Responder tiver método async `respond_to(self, req: &HttpRequest) -> HttpResponse`
                                //    let resp: HttpResponse = responder_box.respond_to(&*req_arc).await;
                                //
                                // 2) se Responder puder ser convertido com `Into<HttpResponse>` (consumindo-o)
                                //    let resp: HttpResponse = (*responder_box).into(); // só se impl Into for concrete types
                                //
                                // 3) se Responder tiver `fn respond(self: Box<Self>, req: &HttpRequest) -> HttpResponse` (sync)
                                //    let resp: HttpResponse = responder_box.respond(&*req_arc);
                                //
                                // Como não tenho a definição da trait `Responder` aqui, deixo esse passo em comentário.
                                //
                                // Por enquanto, vamos tentar uma chamada padrão assíncrona (descomente e ajuste conforme sua trait):
                                //
                                // let mut res = responder_box.respond_to(&*req_arc).await;
                                //
                                // Se precisar eu adapto exatamente conforme a trait `Responder` que você usa.
                                //
                                // --- após termos um HttpResponse (`res`) procedemos aos on_response --- //

                                // usamos req_arc para passar referência às on_response
                                for f in executed.iter() {
                                    // se on_response aceita (&HttpRequest, &mut HttpResponse, &Context)
                                    f.on_response(&*req_arc, &mut res, &*context).await;
                                }

                                // converter `res` em string e escrever no socket
                                let res_string: String = res.into();
                                socket.write_all(res_string.as_bytes()).await?;
                                handled = true;
                                // já movemos req para req_arc, então não usar `req` abaixo
                                break 'routes;
                            }

                            // se saiu por outcome Failure, também precisa executar on_response dos fairings
                            for f in executed.iter() {
                                f.on_response(&req, &mut res, &*context).await;
                            }

                            let res_string: String = res.into();
                            socket.write_all(res_string.as_bytes()).await?;

                            handled = true;
                            break 'routes;
                        }
                    } // end routes

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
