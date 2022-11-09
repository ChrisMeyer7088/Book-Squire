mod hello_world;

use actix_web::{web, Scope};

pub fn routes() -> Scope {
    web::scope("/api").service(hello_world::routes())
}
