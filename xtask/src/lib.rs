use std::{
    env, fs,
    path::{Path, PathBuf},
};
use symlink;

pub fn project_root() -> PathBuf {
    Path::new(
        &env::var("CARGO_MANIFEST_DIR").unwrap_or_else(|_| env!("CARGO_MANIFEST_DIR").to_owned()),
    )
    .ancestors()
    .nth(1)
    .unwrap()
    .to_path_buf()
}

pub async fn move_file(source: &PathBuf, target: &PathBuf) -> std::io::Result<()> {
    // mess
    if !target.parent().unwrap().exists() {
        // check for 'target' folder to exist
        if !target.parent().unwrap().parent().unwrap().exists() {
            fs::create_dir(&target.parent().unwrap().parent().unwrap())?;
        }
        // and then create 'debug' folder (for example)
        fs::create_dir(&target.parent().unwrap())?;
    }

    if let Err(e) = fs::copy(&source, &target) {
        println!("failed to copy: {}", e);
    };

    if let Err(e) = fs::remove_file(source) {
        println!("failed to remove file: {}", e);
    };

    Ok(())
}

pub async fn check_node_modules(package_name: &str) -> Result<(), Box<dyn std::error::Error>> {
    let mut node_modules = project_root();
    node_modules.push("packages");
    node_modules.push(package_name);
    node_modules.push("node_modules");

    if !node_modules.exists() {
        create_npm_process()
            .arg("i")
            .current_dir(node_modules.parent().unwrap())
            .spawn()
            .expect("failed to npm install")
            .await?;
    };

    Ok(())
}

pub fn configure_paths(build_profile: &str) -> (PathBuf, PathBuf) {
    let mut go_server_path: PathBuf = project_root();
    go_server_path.push("packages");
    go_server_path.push("go-server");
    go_server_path.push(go_server_outputname());

    let mut admin_core_path: PathBuf = project_root();
    admin_core_path.push("packages");
    admin_core_path.push("new_admin_tauri");
    admin_core_path.push("src-tauri");
    admin_core_path.push("target");
    admin_core_path.push(build_profile);
    admin_core_path.push(go_server_outputname());

    (go_server_path, admin_core_path)
}

#[cfg(target_os = "linux")]
fn go_server_outputname() -> String {
    String::from("go-server")
}

#[cfg(target_os = "windows")]
fn go_server_outputname() -> String {
    String::from("go-server.exe")
}

pub async fn compile_go_server() -> Result<(), Box<dyn std::error::Error>> {
    tokio::process::Command::new("go")
        .arg("build")
        .arg("-o")
        .arg(go_server_outputname())
        .current_dir("./packages/go-server")
        .spawn()
        .expect("failed to build go server")
        .await?;

    Ok(())
}

#[cfg(target_os = "linux")]
pub fn create_npm_process() -> tokio::process::Command {
    tokio::process::Command::new("npm")
}

#[cfg(target_os = "windows")]
pub fn create_npm_process() -> tokio::process::Command {
    let mut cmd = tokio::process::Command::new("powershell");
    cmd.arg("-c").arg("npm");
    cmd
}

#[cfg(target_os = "linux")]
pub fn create_tauri_process() -> tokio::process::Command {
    tokio::process::Command::new("tauri")
}

#[cfg(target_os = "windows")]
pub fn create_tauri_process() -> tokio::process::Command {
    let mut cmd = tokio::process::Command::new("powershell");
    cmd.arg("-c").arg("tauri");
    cmd
}

pub fn create_symlinks() -> std::io::Result<()> {
    let root = project_root().join("packages");

    if root.join("admin_core").join("src-tauri").exists() {
        remove_symlinks()?;
    };

    symlink::symlink_dir(
        root.join("new_admin_tauri").join("src-tauri/"),
        root.join("admin_core").join("src-tauri"),
    )?;

    Ok(())
}

pub fn remove_symlinks() -> std::io::Result<()> {
    let root = project_root().join("packages");
    symlink::remove_symlink_dir(root.join("admin_core").join("src-tauri"))?;
    Ok(())
}
