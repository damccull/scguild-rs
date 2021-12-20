mod models;

use actix_web::ResponseError;
use anyhow::Context;
pub use models::*;
use sqlx::PgPool;
use uuid::Uuid;

use std::str::FromStr;

use crate::error_chain_fmt;

#[tracing::instrument(name = "Database - Get All Ship Models")]
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

#[tracing::instrument(name = "Database - Get Ship By ID")]
pub async fn get_ship_by_id(pool: &PgPool, id: String) -> Result<ShipModel, DatabaseError> {
    let id = match Uuid::from_str(&id) {
        Ok(x) => x,
        Err(e) => {
            return Err(DatabaseError::UnexpectedError(anyhow::anyhow!(
                "Unable to parse given string as UUID: {:?}",
                e
            )))
        }
    };
    let record = sqlx::query!(
        r#"
        SELECT id, class_name, name, description
        FROM ship_models
        WHERE id = $1
        "#,
        id
    )
    .fetch_one(pool)
    .await
    .context("Failed to execute query")?;

    Ok(ShipModel {
        id: record.id,
        name: record.name,
        description: record.description,
    })
}

#[derive(thiserror::Error)]
pub enum DatabaseError {
    #[error(transparent)]
    UnexpectedError(#[from] anyhow::Error),
}
impl std::fmt::Debug for DatabaseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        error_chain_fmt(self, f)
    }
}
impl ResponseError for DatabaseError {
    fn status_code(&self) -> actix_http::StatusCode {
        match self {
            DatabaseError::UnexpectedError(_) => actix_http::StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}
