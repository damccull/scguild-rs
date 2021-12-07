
use norseline::{telemetry::{get_subscriber, init_subscriber}, configuration::{get_configuration, DatabaseSettings}, application::{Application, get_connection_pool}};
use once_cell::sync::Lazy;
use sqlx::{postgres::PgPoolOptions, ConnectOptions, Connection, Executor, PgConnection, PgPool};
use tracing::log::LevelFilter;
use uuid::Uuid;

static TRACING: Lazy<()> = Lazy::new(|| {
    let default_filter_level = "info".to_string();
    let subscriber_name = "test".to_string();

    if std::env::var("TEST_LOG").is_ok() {
        let subscriber = get_subscriber(subscriber_name, default_filter_level, std::io::stdout);
        init_subscriber(subscriber);
    } else {
        let subscriber = get_subscriber(subscriber_name, default_filter_level, std::io::sink);
        init_subscriber(subscriber);
    }
});

pub struct TestApp {
    pub address: String,
    pub db_pool: PgPool,
    pub port: u16,
}

impl TestApp {
    pub async fn spawn() -> Self {
        //
        // Set up logging
        Lazy::force(&TRACING);

        // Set up config with random db name
        let configuration = {
            let mut c = get_configuration().expect("Failed to load configuration.");
            c.database.database_name = Uuid::new_v4().to_string();
            c.application.port = 0;
            c
        };

        configure_database(&configuration.database).await;

        // Launch the application as a background task
        let application = Application::build(configuration.clone())
            .await
            .expect("Failed to build the application.");

        let application_port = application.port();

        // Run the server in an executor
        let _ = tokio::spawn(application.run_until_stopped());

        let test_app = TestApp {
            address: format!("http://localhost:{}", application_port),
            db_pool: get_connection_pool(&configuration.database),
            port: application_port,
        };

        test_app
    }
}

async fn configure_database(config: &DatabaseSettings) -> PgPool {
    // Create a new database
    let mut connection = PgConnection::connect_with(&config.without_db())
        .await
        .expect("Failed to connect to postgres.");

    connection
        .execute(&*format!(r#"CREATE DATABASE "{}";"#, config.database_name))
        .await
        .expect("Failed to create database.");

    // Create a database pool for the web server, specifying that sqlx logs
    // should be at the `tracing` level.
    let db_connect_options = config
        .with_db()
        .log_statements(LevelFilter::Trace)
        .to_owned();

    let connection_pool = PgPoolOptions::new()
        .connect_timeout(std::time::Duration::from_secs(2))
        .connect_with(db_connect_options)
        .await
        .expect("Failed to connect to Postgres.");

    // Run database migrations
    sqlx::migrate!("./migrations")
        .run(&connection_pool)
        .await
        .expect("Failed to migrate the database.");

    connection_pool
}
