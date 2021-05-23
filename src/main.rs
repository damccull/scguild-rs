use actix_web::{web, App, HttpRequest, HttpServer, Responder};

mod verify_ed25519_signature;

async fn greet(req: HttpRequest) -> impl Responder {
    let name = req.match_info().get("name").unwrap_or("World");
    format!("Hello {}!", &name)
}

async fn api(req: HttpRequest) -> impl Responder {
    let interaction = req.match_info().get("interaction").unwrap_or("bad api");
    format!("API requested path: {}", &interaction)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .wrap(verify_ed25519_signature::VerifyEd25519Signature)
            .route("/", web::get().to(greet))
            .route("/{name}", web::get().to(greet))
            .route("/api/{interaction}", web::get().to(api))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
