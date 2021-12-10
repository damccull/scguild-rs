use actix_web::{HttpRequest, Responder};

pub async fn api(req: HttpRequest) -> impl Responder {
    let interaction = req
        .match_info()
        .get("interaction")
        .unwrap_or("no such interaction");
    format!("API requested path: {}", &interaction)
}
