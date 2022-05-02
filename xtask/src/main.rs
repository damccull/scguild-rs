use std::{
    env,
    fs,
    path::{Path, PathBuf},
    process::{Command, Stdio},
    thread,
    time::Duration,
};

use man::Manual;

type DynError = Box<dyn std::error::Error>;

fn main() {
    if let Err(e) = try_main() {
        eprintln!("{}", e);
        std::process::exit(-1);
    }
}

fn try_main() -> Result<(), DynError> {
    let task = env::args().nth(1);
    match task.as_deref() {
        Some("dockerdb") => docker_db()?,
        Some("migrate") => migrate_db()?,
        Some("dist") => dist()?,
        _ => print_help(),
    }
    Ok(())
}

fn print_help() {
    eprintln!(
        "Tasks:
        
dist            builds application and man pages
dockerdb        starts up a postgres docker container and runs migrations
migrate         runs database migrations"
    )
}

fn dist() -> Result<(), DynError> {
    let _ = fs::remove_dir_all(&dist_dir());
    fs::create_dir_all(&dist_dir())?;

    dist_binary()?;
    dist_manpage()?;

    Ok(())
}

fn dist_binary() -> Result<(), DynError> {
    let cargo = env::var("CARGO").unwrap_or_else(|_| "cargo".to_string());
    let status = Command::new(cargo)
        .current_dir(project_root())
        .args(&["build", "--release"])
        .status()?;

    if !status.success() {
        return Err("cargo build failed".into());
    }

    let mut dst = project_root().join("target/release/norseline");

    let mut destination = dist_dir().join("norseline");

    #[cfg(target_os = "windows")]
    dst.set_extension("exe");
    #[cfg(target_os = "windows")]
    destination.set_extension("exe");

    fs::copy(&dst, destination)?;

    if Command::new("strip")
        .arg("--version")
        .stdout(Stdio::null())
        .status()
        .is_ok()
    {
        eprintln!("stripping the binary");
        let status = Command::new("strip").arg(&dst).status()?;
        if !status.success() {
            return Err("strip failed".into());
        }
    } else {
        eprintln!("No `strip` utility found");
    }

    Ok(())
}

fn docker_db() -> Result<(), DynError> {
    let psql = "psql".to_string();

    check_psql_exists()?;
    check_sqlx_exists()?;

    let db_user = env::var("POSTGRES_USER").unwrap_or_else(|_| "postgres".to_string());
    let db_password = env::var("POSTGRES_PASSWORD").unwrap_or_else(|_| "password".to_string());
    let db_name = env::var("POSTGRES_DB").unwrap_or_else(|_| "norseline".to_string());
    let db_port = env::var("POSTGRES_PORT").unwrap_or_else(|_| "5432".to_string());
    let skip_docker = env::var("SKIP_DOCKER")
        .unwrap_or_else(|_| "false".to_string())
        .parse::<bool>()
        .unwrap_or(false);

    if skip_docker {
        println!("Skipping docker...");
    } else {
        println!("Starting docker image...");
        let status = Command::new("docker")
            .current_dir(project_root())
            .args(&[
                "run",
                "--name",
                "norseline_db",
                "-e",
                &format!("POSTGRES_USER={}", &db_user),
                "-e",
                &format!("POSTGRES_PASSWORD={}", &db_password),
                "-e",
                &format!("POSTGRES_DB={}", &db_name),
                "-p",
                &db_port,
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
            .env("PGPASSWORD", &db_password)
            .args([
                "-h",
                "localhost",
                "-U",
                &db_user,
                "-p",
                &db_port,
                "-d",
                "postgres",
                "-c",
                "\\q",
            ]);

        while check_online.status().is_err() {
            println!("Postgres is still unavailable. Waiting to try again...");
            thread::sleep(Duration::from_millis(1000));
        }
        println!("Docker PostGres server online");
    }

    println!("Migrating database...");

    let migration_status1 = Command::new("sqlx")
        .current_dir(project_root())
        .env(
            "DATABASE_URL",
            format!(
                "postgres://{}:{}@localhost:{}/{}",
                &db_user, &db_password, &db_port, &db_name
            ),
        )
        .args(&["database", "create"])
        .status();

    let migration_status2 = Command::new("sqlx")
        .current_dir(project_root())
        .env(
            "DATABASE_URL",
            format!(
                "postgres://{}:{}@localhost:{}/{}",
                &db_user, &db_password, &db_port, &db_name
            ),
        )
        .args(&["migrate", "--source", "norseline-rs/migrations", "run"])
        .status();

    if migration_status1.is_err() || migration_status2.is_err() {
        return Err("there was a problem running the migration".into());
    }

    println!("Migration completed.");

    Ok(())
}
fn migrate_db() -> Result<(), DynError> {
    todo!();
}

fn dist_manpage() -> Result<(), DynError> {
    let page = Manual::new("norseline-rs")
        .about("Runs a discord bot and website for Star Citizen guild content.")
        .render();
    fs::write(dist_dir().join("norseline-rs.man"), &page)?;
    Ok(())
}

fn project_root() -> PathBuf {
    Path::new(&env!("CARGO_MANIFEST_DIR"))
        .ancestors()
        .nth(1)
        .unwrap()
        .to_path_buf()
}

fn dist_dir() -> PathBuf {
    project_root().join("target/dist")
}

fn check_psql_exists() -> Result<(), DynError> {
    let status = Command::new("psql")
        .current_dir(project_root())
        .args(&["--version"])
        .status();

    match status {
        Err(ref e) if e.kind() == std::io::ErrorKind::NotFound => {
            return Err(
                "Error: 'psql' is not found on the PATH. Please install it to continue.".into(),
            );
        }
        Err(e) => return Err(format!("An unknown error occurred: {}", e).into()),
        _ => {}
    };

    Ok(())
}

fn check_sqlx_exists() -> Result<(), DynError> {
    let status = Command::new("sqlx")
        .current_dir(project_root())
        .args(&["--version"])
        .status();

    match status {
        Err(ref e) if e.kind() == std::io::ErrorKind::NotFound => {
            return Err(
                "Error: 'sqlx' is not found on the PATH. Please install it to continue.".into(),
            );
        }
        Err(e) => return Err(format!("An unknown error occurred: {}", e).into()),
        _ => {}
    };

    Ok(())
}
