use super::handler;
use actix_web::{web, Scope};

pub fn routes() -> Scope {
    web::scope("/hello-world")
        .service(handler::default)
}
