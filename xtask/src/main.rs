use pico_args::Arguments;
use tokio;

use xtask::{
    check_node_modules, compile_go_server, configure_paths, create_npm_process, create_symlinks,
    move_file, remove_symlinks,
};

async fn serve() -> Result<(), Box<dyn std::error::Error>> {
    let (go_server_path, admin_core_path) = configure_paths("debug");

    // Go Server Compile
    compile_go_server().await?;

    move_file(go_server_path, admin_core_path).await?;

    check_node_modules("admin_core").await?;

    create_symlinks()?;

    // npm run serve
    create_npm_process()
        .arg("run")
        .arg("tauri:serve")
        .current_dir("./packages/admin_core")
        .spawn()
        .expect("failed to serve")
        .await?;

    remove_symlinks()?;

    Ok(())
}

async fn build() -> Result<(), Box<dyn std::error::Error>> {
    let (go_server_path, admin_core_path) = configure_paths("release");

    // Go Server Compile
    compile_go_server().await?;

    move_file(go_server_path, admin_core_path).await?;

    check_node_modules("admin_core").await?;

    create_symlinks()?;

    // npm run tauri:build
    create_npm_process()
        .arg("run")
        .arg("tauri:build")
        .current_dir("./packages/admin_core")
        .spawn()
        .expect("failed to build tauri")
        .await?;

    remove_symlinks()?;

    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut args = Arguments::from_env();
    let subcommand = args.subcommand()?.unwrap_or_default();

    match subcommand.as_str() {
        "serve" => {
            serve().await?;
            Ok(())
        }
        "build" => {
            build().await?;
            Ok(())
        }
        "server" => {
            let profile = args
                .opt_value_from_str("--profile")?
                .unwrap_or("debug".to_string());
            let (go_server_path, admin_core_path) = configure_paths(&profile);
            compile_go_server().await?;
            move_file(go_server_path, admin_core_path).await?;
            Ok(())
        }
        _ => {
            eprintln!(
                "\
cargo xtask
Run custom build command.
USAGE:
    cargo xtask <SUBCOMMAND>
SUBCOMMANDS:
    serve
    build
    server"
            );
            Ok(())
        }
    }
}
