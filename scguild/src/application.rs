use std::net::TcpListener;

use axum::{
    extract::FromRef,
    middleware,
    routing::{get, IntoMakeService},
    Router, Server,
};
use axum_session::{SessionConfig, SessionLayer, SessionRedisPool, SessionStore};
use hyper::server::conn::AddrIncoming;
use secrecy::{ExposeSecret, Secret};
use sqlx::{postgres::PgPoolOptions, PgPool};

use crate::{
    configuration::{DatabaseSettings, Settings},
    telemetry::RouterExt,
};

pub type AppServer = Server<AddrIncoming, IntoMakeService<Router>>;

pub struct Application {
    port: u16,
    server: AppServer,
}

impl Application {
    pub async fn build(configuration: Settings) -> Result<Self, anyhow::Error> {
        // Get database pool
        let db_pool = get_db_pool(&configuration.database);

        // Build a redis session
        let redis = redis::Client::open(configuration.redis.uri.expose_secret().as_str())?;
        let session_config = SessionConfig::new();
        let session_store =
            SessionStore::<SessionRedisPool>::new(Some(redis.into()), session_config);

        let address = format!(
            "{}:{}",
            configuration.application.host, configuration.application.port
        );

        let listener = TcpListener::bind(address.to_string()).map_err(|e| {
            tracing::error!("failed to bind port {}", address);
            e
        })?;

        let port = listener.local_addr().unwrap().port();

        let server = run(
            listener,
            db_pool,
            configuration.application.base_url,
            configuration.application.hmac_secret,
            session_store,
        );

        Ok(Self { port, server })
    }

    pub fn port(&self) -> u16 {
        self.port
    }

    pub async fn run_until_stopped(self) -> hyper::Result<()> {
        self.server.await
    }
}

pub fn get_db_pool(configuration: &DatabaseSettings) -> PgPool {
    PgPoolOptions::new()
        .acquire_timeout(std::time::Duration::from_secs(2))
        .connect_lazy_with(configuration.with_db())
}
pub fn run(
    listener: TcpListener,
    db_pool: PgPool,
    base_url: String,
    hmac_secret: Secret<String>,
    session_store: SessionStore<SessionRedisPool>,
) -> AppServer {
    // Build the app state
    let app_state = AppState {
        db_pool,
        base_url: ApplicationBaseUrl(base_url),
    };

    // Routes that don't need a session applied
    let router_no_session = Router::new().route("/health_check", get(health_check));

    // Admin section routes
    let router_admin = Router::new()
        .route("/admin/dashboard", get(admin_dashboard))
        .layer(middleware::from_fn(reject_anonymous_users));

    // All routes that care about a session
    let router_with_session = Router::new()
        .route("/", get(home))
        .merge(router_admin)
        .layer(SessionLayer::new(session_store));

    // Create a top-level router that matches all routes
    let app = Router::new()
        .merge(router_no_session)
        .merge(router_with_session)
        .add_axum_tracing_layer()
        .with_state(app_state);

    // Start the axum server using the supplied listener
    axum::Server::from_tcp(listener)
        .expect("failed to create server from listener")
        .serve(app.into_make_service())
}

#[derive(Clone)]
pub struct AppState {
    db_pool: PgPool,
    base_url: ApplicationBaseUrl,
}

impl FromRef<AppState> for PgPool {
    fn from_ref(app_state: &AppState) -> Self {
        app_state.db_pool.clone()
    }
}

impl FromRef<AppState> for ApplicationBaseUrl {
    fn from_ref(app_state: &AppState) -> Self {
        app_state.base_url.clone()
    }
}

#[derive(Clone)]
pub struct ApplicationBaseUrl(pub String);

#[derive(Clone)]
pub struct HmacSecret(pub Secret<String>);
