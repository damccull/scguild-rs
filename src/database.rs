mod models;

pub use models::*;
use sqlx::PgPool;


pub struct StarCitizenRepository {
    pool: &PgPool
}
impl StarCitizenRepository {}
#[tracing::instrument(name = "Database - Get all ship models")]
pub async fn all_ship_models(pool: &PgPool) -> Result<Vec<ShipModel>, anyhow::Error> {
    Ok(sqlx::query!(
        r#"
        SELECT id, name, description
        FROM ship_models
        "#
    )
    .fetch_all(pool)
    .await
    .map_err(|e| anyhow::anyhow!(format!("Failed to execute query: {:?}", e)))?
    .iter()
    .map(|row| ShipModel {
        id: row.id.to_owned(),
        name: row.name.to_owned(),
        description: row.description.to_owned(),
    })
    .collect::<Vec<_>>())
}
