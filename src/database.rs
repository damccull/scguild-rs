mod models;

pub use models::*;
use sqlx::PgPool;
use uuid::Uuid;

use crate::discord::api::DiscordApiError;

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
    .collect::<Vec<_>>();
    Ok(r)
}

pub async fn get_ship_by_id(pool: &PgPool, query: String) -> Option<ShipModel> {
    let query_uuid = match Uuid::from_str(&self.ship_model) {
        Ok(x) => x,
        Err(e) => {
            return Err(DiscordApiError::UnexpectedError(anyhow::anyhow!(
                "Unable to parse given string as UUID: {:?}",
                e
            )))
        }
    };
    let r = sqlx::query!(
        r#"
        SELECT id, class_name, name, description
        FROM ship_models
        WHERE id = $1
        "#,
    )
    .fetch_optional(pool)
    .await
    .context("Failed to execute query")?
    .map(|row| ShipModel {
        id: row.id,
        name: row.name,
    });

    Ok(r)
}
