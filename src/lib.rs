#![allow(unused)]

use std::net::TcpListener;

use actix_web::{App, HttpRequest, HttpResponse, HttpServer, Responder, dev::Server, web::{self, scope}};

mod crypto;
mod entities;

/// Returns a Server without awaiting it. This allows for integration testing.
pub fn run(listener: TcpListener) -> Result<Server, std::io::Error> {
    let server = HttpServer::new(|| {
        App::new()
            .route("/health_check", web::get().to(health_check))
            .route("/", web::get().to(greet))
            .route("/{name}", web::get().to(greet))
            .service(
                web::scope("/api").service(
                    web::scope("/discord")
                        .wrap(crypto::VerifyEd25519Signature)
                        .route("/{interaction}", web::get().to(api)),
                )
                .route("/{interaction}", web::get().to(api)),
            )
    })
    .listen(listener)?
    .run();
    
    Ok(server)
}

async fn health_check() -> HttpResponse {
    HttpResponse::Ok().finish()
}

async fn greet(req: HttpRequest) -> impl Responder {
    let name = req.match_info().get("name").unwrap_or("World");
    format!("Hello {}!", &name)
}

async fn api(req: HttpRequest) -> impl Responder {
    let interaction = req.match_info().get("interaction").unwrap_or("bad api");
    format!("API requested path: {}", &interaction)
}