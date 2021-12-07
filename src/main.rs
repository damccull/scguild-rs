use norseline::{
    application::Application,
    configuration::get_configuration,
    telemetry::{get_subscriber, init_subscriber},
};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Set up logging using tracing, tracing-subscriber, and tracing-bunyan-formatter
    let subscriber = get_subscriber("norseline".into(), "info".into(), std::io::stdout);
    init_subscriber(subscriber);

    // Read the configuration, panicking if it can't be read
    let configuration = get_configuration().expect("Failed to read the configuration.");

    // Create the application and launch it
    let application = Application::build(configuration).await?;
    application.run_until_stopped().await?;
    Ok(())
}
