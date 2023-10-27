use http::StatusCode;

#[allow(clippy::let_with_type_underscore)]
/// Return HTTP status code OK (200) to act as a health check
#[tracing::instrument(name = "Health Check")]
pub async fn health_check() -> StatusCode {
    StatusCode::OK
}
