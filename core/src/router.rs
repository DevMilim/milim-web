use std::io::Write;

use crate::request::{HttpRequest, Method, Resource};

pub struct Router;
impl Router {
    pub fn route(req: HttpRequest, stream: &mut impl Write) {
        use Method::*;
        use Resource::*;
        match req.method {
            Get => match &req.resource {
                Path(_) => todo!(),
            },
            Post => todo!(),
            Uninitialized => todo!(),
        }
    }
}
