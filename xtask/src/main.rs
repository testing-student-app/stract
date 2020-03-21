use pico_args::Arguments;

use std::fs;
use std::path::{Path, PathBuf};
use xtask::project_root;

async fn move_go_server(source: PathBuf, target: PathBuf) -> std::io::Result<()> {
    if !target.exists() {
        fs::create_dir(&target)?;
    }

    fs::copy(&source, &target)?;
    fs::remove_file(source)?;

    Ok(())
}

async fn serve() {
    let mut go_server_root: PathBuf = project_root();
    go_server_root.push("go-server");

    let mut admin_tauri: PathBuf = project_root();
    admin_tauri.push("frontend-admin-taur");

    // Go Server Compile
    tokio::spawn(async move {});

    move_go_server().await;

    // npm run tauri:serve
    tokio::spawn(async move {});
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut args = Arguments::from_env();
    let subcommand = args.subcommand()?.unwrap_or_default();

    match subcommand.as_str() {
        "serve" => {
            serve().await;
            Ok(())
        }
        "build" => Ok(()),
        _ => {
            eprintln!(
                "\
cargo xtask
Run custom build command.
USAGE:
    cargo xtask <SUBCOMMAND>
SUBCOMMANDS:
    serve
    build"
            );
            Ok(())
        }
    }
}
