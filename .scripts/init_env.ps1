Write-Output "Setting up enviromental Variables"
# setup relative paths
$dist_path = "packages\admin_core\dist"
$src_path = "packages\new_admin_tauri\src-tauri"

# check to see if path variables are directories
if ((Test-Path $dist_path -PathType Any) -Or (Test-Path $src_path -PathType Any)) {
    # convert relative paths to absolute paths.
    # put these absolute paths in enviromental variables
    $env:TAURI_DIST_DIR = Resolve-Path $dist_path
    $env:TAURI_DIR = Resolve-Path $src_path
    Write-Output "Variables set, ready to work!"

}
else {
    Write-Output "Variables are not setup properly. Please run from Tauri Root directory"
}
