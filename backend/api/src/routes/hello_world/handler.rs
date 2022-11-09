use super::types::Message;
use actix_web::{get, web, Responder};

#[get("")]
pub async fn default() -> impl Responder {
  web::Json(Message {
    text: "Hello World!".to_string(),
  })
}
