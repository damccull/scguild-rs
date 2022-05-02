use std::{env, process::Command, thread, time::Duration};

use crate::{check_psql_exists, check_sqlx_exists, project_root, DbConfig, DynError};

pub fn docker_db() -> Result<(), DynError> {
    let psql = "psql".to_string();
    let docker = "docker".to_string();

    check_psql_exists()?;

    // Set up needed variables from the environment or use defaults
    let db_config = DbConfig::get_config();

    let skip_docker = env::var("SKIP_DOCKER")
        .unwrap_or_else(|_| "false".to_string())
        .parse::<bool>()
        .unwrap_or(false);

    if skip_docker {
        println!("Skipping docker...");
    } else {
        println!("Starting docker image...");
        let _status = Command::new(docker)
            .current_dir(project_root())
            .args(&[
                "run",
                "--name",
                "norseline_db",
                "-e",
                &format!("POSTGRES_USER={}", &db_config.username()),
                "-e",
                &format!("POSTGRES_PASSWORD={}", &db_config.password()),
                "-e",
                &format!("POSTGRES_DB={}", &db_config.db_name()),
                "-p",
                &db_config.db_port(),
                "-d",
                "postgres",
                "postgres",
                "-N",
                "1000",
            ])
            .status()?;

        let mut check_online = Command::new(psql);
        let check_online = check_online
            .current_dir(project_root())
            .env("PGPASSWORD", &db_config.password())
            .args([
                "-h",
                "localhost",
                "-U",
                &db_config.username(),
                "-p",
                &db_config.db_port(),
                "-d",
                "postgres",
                "-c",
                "\\q",
            ]);

        while check_online.status().is_err() {
            println!("Postgres is still unavailable. Waiting to try again...");
            thread::sleep(Duration::from_millis(1000));
        }
        println!("Docker Postgres server online");
    }

    // Migrate the database automatically as part of initialization
    migrate_db()?;

    Ok(())
}

pub fn migrate_db() -> Result<(), DynError> {
    check_sqlx_exists()?;

    // Set up needed variables from the environment or use defaults
    let db_config = DbConfig::get_config();

    println!("Migrating database...");

    let migration_status1 = Command::new("sqlx")
        .current_dir(project_root())
        .env(
            "DATABASE_URL",
            format!(
                "postgres://{}:{}@localhost:{}/{}",
                &db_config.username(),
                &db_config.password(),
                &db_config.db_port(),
                &db_config.db_name()
            ),
        )
        .args(&["database", "create"])
        .status();

    let migration_status2 = Command::new("sqlx")
        .current_dir(project_root().join("norseline-rs"))
        .env(
            "DATABASE_URL",
            format!(
                "postgres://{}:{}@localhost:{}/{}",
                &db_config.username(),
                &db_config.password(),
                &db_config.db_port(),
                &db_config.db_name()
            ),
        )
        .args(&["migrate", "run"])
        .status();

    if migration_status1.is_err() || migration_status2.is_err() {
        return Err("there was a problem running the migration".into());
    }

    println!("Migration completed.");

    Ok(())
}
