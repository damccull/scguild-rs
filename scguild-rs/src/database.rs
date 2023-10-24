mod models;

use actix_web::ResponseError;
use anyhow::Context;
pub use models::*;
use sqlx::PgPool;
use uuid::Uuid;

use crate::error_chain_fmt;

#[tracing::instrument(name = "Database - Get All Ship Models", skip(pool))]
pub async fn all_ship_models(pool: &PgPool) -> Result<Vec<ShipModel>, anyhow::Error> {
    Ok(sqlx::query!(
        r#"
        SELECT id, name, description, class_name, manufacturer_id, focus, career, role,
        size, cargo_amount, crew, weapon_crew, operations_crew, mass, is_spaceship, is_vehicle, is_gravlev
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
        class_name: row.class_name.to_owned(),
        manufacturer_id: row.manufacturer_id.to_owned(),
        focus: row.focus.to_owned(),
        career: row.career.to_owned(),
        role: row.role.to_owned(),
        size: row.size.to_owned(),
        cargo: row.cargo_amount.to_owned(),
        crew: row.crew.to_owned(),
        weapon_crew: row.weapon_crew.to_owned(),
        operations_crew: row.operations_crew.to_owned(),
        mass: row.mass.to_owned(),
        is_spaceship: row.is_spaceship.to_owned(),
        is_vehicle: row.is_vehicle.to_owned(),
        is_gravlev: row.is_gravlev.to_owned(),
    })
    .collect::<Vec<_>>())
}

#[tracing::instrument(name = "Database - Get Ship By ID", skip(pool))]
pub async fn get_ship_by_id(pool: &PgPool, id: Uuid) -> Result<ShipModel, DatabaseError> {
    let record = sqlx::query!(
        r#"
        SELECT id, name, description, class_name, manufacturer_id, focus, career, role,
        size, cargo_amount, crew, weapon_crew, operations_crew, mass, is_spaceship, is_vehicle, is_gravlev
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
        class_name: record.class_name.to_owned(),
        manufacturer_id: record.manufacturer_id.to_owned(),
        focus: record.focus.to_owned(),
        career: record.career.to_owned(),
        role: record.role.to_owned(),
        size: record.size.to_owned(),
        cargo: record.cargo_amount.to_owned(),
        crew: record.crew.to_owned(),
        weapon_crew: record.weapon_crew.to_owned(),
        operations_crew: record.operations_crew.to_owned(),
        mass: record.mass.to_owned(),
        is_spaceship: record.is_spaceship.to_owned(),
        is_vehicle: record.is_vehicle.to_owned(),
        is_gravlev: record.is_gravlev.to_owned(),
    })
}

pub async fn get_ships_by_model_name(
    pool: &PgPool,
    name: String,
) -> Result<Vec<ShipModel>, DatabaseError> {
    let record = sqlx::query!(
        r#"
        SELECT id, name, description, class_name, manufacturer_id, focus, career, role,
        size, cargo_amount, crew, weapon_crew, operations_crew, mass, is_spaceship, is_vehicle, is_gravlev
        FROM ship_models
        WHERE lower(name) LIKE lower($1)
        "#,
        format!("%{}%", name)
    )
    .fetch_all(pool)
    .await
    .map_err(|e| {
        tracing::error!("No record found for {}: {}", name, e);
        DatabaseError::RecordNotFoundError(name.to_string())
    })?
    .iter()
    .map(|row| ShipModel {
        id: row.id.to_owned(),
        name: row.name.to_owned(),
        description: row.description.to_owned(),
        class_name: row.class_name.to_owned(),
        manufacturer_id: row.manufacturer_id.to_owned(),
        focus: row.focus.to_owned(),
        career: row.career.to_owned(),
        role: row.role.to_owned(),
        size: row.size.to_owned(),
        cargo: row.cargo_amount.to_owned(),
        crew: row.crew.to_owned(),
        weapon_crew: row.weapon_crew.to_owned(),
        operations_crew: row.operations_crew.to_owned(),
        mass: row.mass.to_owned(),
        is_spaceship: row.is_spaceship.to_owned(),
        is_vehicle: row.is_vehicle.to_owned(),
        is_gravlev: row.is_gravlev.to_owned(),
    })
    .collect::<Vec<_>>();

    Ok(record)
}

pub async fn insert_user(pool: &PgPool, user: User) -> Result<Uuid, DatabaseError> {
    let mut transaction = pool
        .begin()
        .await
        .context("Failed to get a Postgres connection from the pool.")?;

    let _response = sqlx::query!(
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
