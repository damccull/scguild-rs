use actix_web::{http::header, web, HttpRequest, HttpResponse, Responder};
use twilight_model::application::{callback::InteractionResponse, interaction::Interaction};

pub async fn discord_api(_req: HttpRequest, interaction: web::Json<Interaction>) -> impl Responder {
    match interaction.0 {
        Interaction::Ping(_) => {
            let response = InteractionResponse::Pong;

            let x = serde_json::to_vec(&response);
            dbg!("DEBUG DEBUG DEBUG",&response,x);
            HttpResponse::Ok()
                .append_header(header::ContentType(mime::APPLICATION_JSON))
                .json(response)
        }
        Interaction::ApplicationCommand(_) => HttpResponse::Ok().finish(),
        _ => HttpResponse::BadRequest().finish(),
    }
}
