use std::{
    env, fs,
    path::{Path, PathBuf},
    process::{Command, Stdio},
    thread,
    time::Duration,
};

use man::Manual;
use xtask::DbConfig;

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
    //dist_manpage()?;

    Ok(())
}

fn dist_binary() -> Result<(), DynError> {
    // Get the `cargo` command and then build the release
    let cargo = env::var("CARGO").unwrap_or_else(|_| "cargo".to_string());
    let status = Command::new(cargo)
        .current_dir(project_root())
        .args(&["build", "--release"])
        .status()?;

    if !status.success() {
        return Err("cargo build failed".into());
    }

    // Set file paths based on the architecture
    #[allow(unused_mut)]
    let mut distributable = project_root().join("target/release/norseline");

    #[allow(unused_mut)]
    let mut destination = dist_dir().join("norseline");

    #[cfg(windows)]
    distributable.set_extension("exe");
    #[cfg(windows)]
    destination.set_extension("exe");

    // Copy the binary
    fs::copy(&distributable, destination)?;

    // Copy config files
    let config_src = project_root().join("norseline-rs/configuration");
    let config_dest = dist_dir().join("configuration");
    fs::create_dir(config_dest)?;

    for f in fs::read_dir(config_src)? {
        let f = f?;
        if f.file_name().eq("local.yml") {
            continue;
        }
        fs::copy(
            f.path(),
            dist_dir().join("configuration").join(f.file_name()),
        )?;
    }

    // Strip the binary
    if Command::new("strip")
        .arg("--version")
        .stdout(Stdio::null())
        .status()
        .is_ok()
    {
        eprintln!("stripping the binary");
        let status = Command::new("strip").arg(&distributable).status()?;
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

fn migrate_db() -> Result<(), DynError> {
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
        .args(&["migrate", "--source", "norseline-rs/migrations", "run"])
        .status();

    if migration_status1.is_err() || migration_status2.is_err() {
        return Err("there was a problem running the migration".into());
    }

    println!("Migration completed.");

    Ok(())
}

fn _dist_manpage() -> Result<(), DynError> {
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
