#![allow(unused)]

use std::net::TcpListener;
pub struct TestApp {
    pub address: String,
    pub db_pool: PgPool,
}

pub async fn spawn_app() -> TestApp {
    let listener = TcpListener::bind("127.0.0.1:0").expect("Failed to bind a random port.");

    let port = listener.local_addr().unwrap().port();

    let address = format!("http://127.0.0.1:{}", port);

    let mut configuration = get_configuration().expect("Failed to read configuration.");
    // Randomize the database name to guarantee tests will not interact
    configuration.database.database_name = Uuid::new_v4().to_string();
    // Create the database in postgres
    let db_pool = configure_database(&configuration.database).await;

    // let db_pool = PgPool::connect(&configuration.database.connection_string())
    //     .await
    //     .expect("Failed to connect to Postgres.");

    let server = run(listener, db_pool.clone()).expect("Failed to bind address.");
    let _ = tokio::spawn(server);

    TestApp { address, db_pool }
}

pub async fn configure_database(config: &DatabaseSettings) -> PgPool {
    // Create the database
    let mut connection = PgConnection::connect(&config.connection_string_without_db())
        .await
        .expect("Failed to connect to Postgres.");
    connection
        .execute(&*format!(r#"CREATE DATABASE "{}";"#, config.database_name))
        .await
        .expect("Failed to create database.");

    // Migrate the database
    let connection_pool = PgPool::connect(&config.connection_string())
        .await
        .expect("Failed to connect to Postgre.");
    sqlx::migrate!("./migrations")
        .run(&connection_pool)
        .await
        .expect("Failed to migrate the database.");

    connection_pool
}


pub const TEST_PUBLIC_KEY: &str = "172676b110ea0e14d6f41c50cf6b82dbf789cfeb5057eafc4ed58b2d49d98c75edd6964ba8ce8ab6d945581056b553a8f3dcef978a2bcfa8879ea7747384c10f";
pub const TEST_TIMESTAMP: &str = "123";
pub const TEST_MESSAGE: &str = "Test message";
