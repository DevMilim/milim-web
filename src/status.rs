#[derive(PartialEq, Clone, Debug)]
pub enum StatusCode {
    Ok,
    BadRequest,
    NotFound,
    MethodNotAllowed,
    InternalServerError,
    NotAutorized,
}
