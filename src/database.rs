mod models;

use actix_web::ResponseError;
use anyhow::Context;
pub use models::*;
use sqlx::{Executor, PgPool};
use uuid::Uuid;

use crate::error_chain_fmt;

#[tracing::instrument(name = "Database - Get All Ship Models", skip(pool))]
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

#[tracing::instrument(name = "Database - Get Ship By ID", skip(pool))]
pub async fn get_ship_by_id(pool: &PgPool, id: Uuid) -> Result<ShipModel, DatabaseError> {
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
    .map_err(|e| {
        tracing::error!("No record found for {}: {}", id, e);
        DatabaseError::RecordNotFoundError(id.to_string())
    })?;

    Ok(ShipModel {
        id: record.id,
        name: record.name,
        description: record.description,
    })
}

pub async fn insert_user(pool: &PgPool, user: User) -> Result<Uuid, DatabaseError> {
    let mut transaction = pool
        .begin()
        .await
        .context("Failed to get a Postgres connection from the pool.")?;

    let response = sqlx::query!(
        r#"
        INSERT INTO users (id, discord_id)
        VALUES ($1, $2)
        "#,
        user.id,
        user.discord_id,
    )
    .execute(&mut transaction)
    .await
    .context("tesT")?;

    Ok(user.id)
}

#[derive(thiserror::Error)]
pub enum DatabaseError {
    #[error("Record not found for: {0}")]
    RecordNotFoundError(String),
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
            DatabaseError::RecordNotFoundError(_) => actix_http::StatusCode::NOT_FOUND,
            DatabaseError::UnexpectedError(_) => actix_http::StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}
