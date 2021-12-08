use actix_web::{HttpRequest, Responder};

pub async fn discord_api(req: HttpRequest) -> impl Responder {
    let interaction = req
        .match_info()
        .get("interaction")
        .unwrap_or("no such interation");
    format!("Discord interaction requested: {}", &interaction)
}
