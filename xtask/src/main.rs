use std::{
    env, fs,
    path::{Path, PathBuf},
    process::{Command, Stdio},
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
        Some("dist") => dist()?,
        _ => print_help(),
    }
    Ok(())
}

fn print_help() {
    eprintln!(
        "Tasks:
        
dist            builds application and man pages"
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

    #[cfg(windows)]
    dst.set_extension("exe");
    #[cfg(windows)]
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
