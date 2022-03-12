use actix_web::{web, HttpRequest, Responder};

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(web::scope("/api").route("/v1/{interaction}", web::get().to(api)));
}

pub async fn api(req: HttpRequest) -> impl Responder {
    let interaction = req
        .match_info()
        .get("interaction")
        .unwrap_or("no such interaction");
    format!("API requested path: {}", &interaction)
}
