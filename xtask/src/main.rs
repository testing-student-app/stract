extern crate clap;

use clap::{crate_version, App, Arg, ArgMatches, SubCommand};
use tokio;

use xtask::{
    check_node_modules, compile_go_server, configure_paths, create_npm_process,
    create_tauri_process, move_file,
};

async fn dev(matches: &ArgMatches<'_>) -> Result<(), Box<dyn std::error::Error>> {
    let (go_server_path, admin_core_path) = configure_paths("debug");

    if matches.is_present("admin-core") {
        check_node_modules("admin_core").await?;
        create_npm_process()
            .arg("run")
            .arg("serve")
            .current_dir("./packages/admin_core")
            .spawn()
            .expect("failed to serve admin_core")
            .await?;
    } else if matches.is_present("client-core") {
        check_node_modules("client_core").await?;
        create_npm_process()
            .arg("run")
            .arg("serve")
            .current_dir("./packages/client_core")
            .spawn()
            .expect("failed to serve client_core")
            .await?;
    } else if matches.is_present("admin-tauri") {
        create_tauri_process()
            .arg("dev")
            .current_dir("./packages/new_admin_tauri")
            .spawn()
            .expect("failed to dev new_admin_tauri")
            .await?;
    } else if matches.is_present("client-tauri") {
        create_tauri_process()
            .arg("dev")
            .current_dir("./packages/client_tauri")
            .spawn()
            .expect("failed to dev client_tauri")
            .await?;
    } else if matches.is_present("go-server") {
        compile_go_server().await?;
        if matches.is_present("move") {
            move_file(&go_server_path, &admin_core_path).await?;
        }
    } else {
        /* This is not working cuz vue-plugin-tauri was not updated in the
        time of this commit */

        // Go Server Compile
        // compile_go_server().await?;
        // move_file(&go_server_path, &admin_core_path).await?;

        // check_node_modules("admin_core").await?;

        // create_symlinks()?;

        // // npm run tauri:serve
        // create_npm_process()
        //     .arg("run")
        //     .arg("tauri:serve")
        //     .current_dir("./packages/admin_core")
        //     .spawn()
        //     .expect("failed to serve tauri")
        //     .await?;

        // remove_symlinks()?;
    }

    Ok(())
}

async fn build(matches: &ArgMatches<'_>) -> Result<(), Box<dyn std::error::Error>> {
    let (go_server_path, admin_core_path) = configure_paths("release");

    if matches.is_present("admin-core") {
        check_node_modules("admin_core").await?;
        create_npm_process()
            .arg("run")
            .arg("build")
            .current_dir("./packages/admin_core")
            .spawn()
            .expect("failed to build admin_core")
            .await?;
    } else if matches.is_present("client-core") {
        check_node_modules("client_core").await?;
        create_npm_process()
            .arg("run")
            .arg("build")
            .current_dir("./packages/client_core")
            .spawn()
            .expect("failed to build client_core")
            .await?;
    } else if matches.is_present("admin-tauri") {
        create_tauri_process()
            .arg("build")
            .current_dir("./packages/new_admin_tauri")
            .spawn()
            .expect("failed to build new_admin_tauri")
            .await?;
    } else if matches.is_present("client-tauri") {
        create_tauri_process()
            .arg("build")
            .current_dir("./packages/client_tauri")
            .spawn()
            .expect("failed to build client_tauri")
            .await?;
    } else if matches.is_present("go-server") {
        compile_go_server().await?;
        if matches.is_present("move") {
            move_file(&go_server_path, &admin_core_path).await?;
        }
    } else {
        /* This is not working cuz vue-plugin-tauri was not updated in the
        time of this commit */

        // Go Server Compile
        // compile_go_server().await?;
        // move_file(&go_server_path, &admin_core_path).await?;

        // check_node_modules("admin_core").await?;

        // create_symlinks()?;

        // // npm run tauri:build
        // create_npm_process()
        //     .arg("run")
        //     .arg("tauri:build")
        //     .current_dir("./packages/admin_core")
        //     .spawn()
        //     .expect("failed to build tauri")
        //     .await?;

        // remove_symlinks()?;
    }

    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let author = "Reidond <reidond@gmail.com>";
    let common_args: Vec<Arg> = vec![
        Arg::with_name("admin-core")
            .long("admin-core")
            .help("admin_core"),
        Arg::with_name("client-core")
            .long("client-core")
            .help("client_core"),
        Arg::with_name("admin-tauri")
            .long("admin-tauri")
            .help("admin_tauri"),
        Arg::with_name("client-tauri")
            .long("client-tauri")
            .help("client_tauri"),
        Arg::with_name("go-server")
            .long("go-server")
            .help("go-server"),
        Arg::with_name("move").long("move").help("move"),
    ];
    let matches = App::new("xtask")
        .version(crate_version!())
        .author(author)
        .about("Build system")
        .subcommand(
            SubCommand::with_name("dev")
                .about("dev")
                .version(crate_version!())
                .author(author)
                .args(&common_args),
        )
        .subcommand(
            SubCommand::with_name("build")
                .about("build")
                .version(crate_version!())
                .author(author)
                .args(&common_args),
        )
        .get_matches();

    if let Some(matches) = matches.subcommand_matches("dev") {
        dev(matches).await?;
    } else if let Some(matches) = matches.subcommand_matches("build") {
        build(matches).await?;
    }

    Ok(())
}
