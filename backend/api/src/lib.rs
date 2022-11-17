mod routes;
mod db;

use std::env;
use actix_web::{App, HttpServer};
use listenfd::ListenFd;
use migration::sea_orm::{DatabaseConnection};

#[derive(Debug, Clone)]
struct AppState {
    conn: DatabaseConnection,
}

#[actix_web::main]
async fn start() -> std::io::Result<()> {
    dotenvy::dotenv().ok();
    let host = env::var("HOST").expect("HOST is not set in .env file");
    let port = env::var("PORT").expect("PORT is not set in .env file");
    let server_url = format!("{}:{}", host, port);

    let mut server = HttpServer::new(move || {
        App::new()
            .service(routes::routes())
    });

    let mut listenfd = ListenFd::from_env();
    server = match listenfd.take_tcp_listener(0)? {
        Some(listener) => server.listen(listener)?,
        None => server.bind(&server_url)?,
    };

    let conn = db::establish_connection().await.unwrap();
    let state = AppState { conn };

    println!("Starting server at {}", server_url);
    server.run().await?;

    Ok(())
}


pub fn main() {
    let result = start();
    
    if let Some(err) = result.err() {
        println!("Error: {}", err);
    }
}
