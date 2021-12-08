use actix_web::{http::header, web, HttpRequest, HttpResponse, Responder};
use twilight_model::application::{callback::InteractionResponse, interaction::Interaction};

pub async fn discord_api(_req: HttpRequest, interaction: web::Json<Interaction>) -> impl Responder {
    match interaction.0 {
        Interaction::Ping(_) => HttpResponse::Ok()
            .append_header(header::ContentType(mime::APPLICATION_JSON))
            .json(InteractionResponse::Pong),
        Interaction::ApplicationCommand(_) => HttpResponse::Ok().finish(),
        _ => HttpResponse::BadRequest().finish(),
    }
}
