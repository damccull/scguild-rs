#![allow(unused)]
#[macro_use]
extern crate diesel;

use std::net::TcpListener;

use actix_web::{
    dev::Server,
    web::{self, scope},
    App, HttpRequest, HttpResponse, HttpServer, Responder,
};

mod crypto;
pub mod database;
pub mod discord;
pub mod entities;
pub mod fleet;

/// Returns a `Server` without awaiting it. This allows for integration testing.
///
/// Takes a `TcpListener`, expecting it to already be bound. This allows for easy integration testing.
pub fn run(listener: TcpListener) -> Result<Server, std::io::Error> {
    let server = HttpServer::new(|| {
        App::new()
            .route("/health_check", web::get().to(health_check))
            .service(
                web::scope("/api")
                    .service(
                        web::scope("/discord")
                            .wrap(crypto::VerifyEd25519Signature)
                            .route("/{interaction}", web::get().to(discord_api)),
                    )
                    .route("/v1/{interaction}", web::get().to(api)),
            )
    })
    .listen(listener)?
    .run();

    Ok(server)
}

async fn health_check() -> HttpResponse {
    HttpResponse::Ok().finish()
}

async fn api(req: HttpRequest) -> impl Responder {
    let interaction = req
        .match_info()
        .get("interaction")
        .unwrap_or("no such interaction");
    format!("API requested path: {}", &interaction)
}

async fn discord_api(req: HttpRequest) -> impl Responder {
    let interaction = req
        .match_info()
        .get("interaction")
        .unwrap_or("no such interation");
    format!("Discord interaction requested: {}", &interaction)
}
