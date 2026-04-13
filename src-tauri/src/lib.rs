// ============================================================
// auto-install-openclaw — Tauri Backend
// ============================================================

#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod github;
mod installer;
mod node;

use github::DeviceCodeResponse;

// ------ Tauri Commands ------

#[tauri::command]
async fn github_device_code() -> Result<DeviceCodeResponse, String> {
    github::request_device_code().await
}

#[tauri::command]
async fn github_poll_token(device_code: String, interval: u64) -> Result<String, String> {
    github::poll_for_token(&device_code, interval).await
}

#[tauri::command]
async fn check_copilot(token: String) -> Result<bool, String> {
    github::check_copilot_subscription(&token).await
}

#[tauri::command]
async fn get_default_install_path() -> Result<String, String> {
    let home = dirs::home_dir().ok_or("Cannot find home directory")?;
    Ok(home.join(".openclaw").to_string_lossy().into_owned())
}

#[tauri::command]
async fn install_check_node(_path: String, _token: String, _autostart: bool) -> Result<String, String> {
    match node::check_node()? {
        Some(version) => Ok(format!("Node.js {} 발견", version)),
        None => Ok("Node.js 미설치 — 설치가 필요합니다".into()),
    }
}

#[tauri::command]
async fn install_node(_path: String, _token: String, _autostart: bool) -> Result<String, String> {
    node::install_node().await
}

#[tauri::command]
async fn install_openclaw(_path: String, _token: String, _autostart: bool) -> Result<String, String> {
    // Run in blocking thread since it uses std::process::Command
    tokio::task::spawn_blocking(|| installer::install_openclaw())
        .await
        .map_err(|e| format!("Task error: {}", e))?
}

#[tauri::command]
async fn install_config(_path: String, token: String, _autostart: bool) -> Result<String, String> {
    installer::generate_config(&token)
}

#[tauri::command]
async fn install_start_gateway(_path: String, _token: String, _autostart: bool) -> Result<String, String> {
    tokio::task::spawn_blocking(|| installer::start_gateway())
        .await
        .map_err(|e| format!("Task error: {}", e))?
}

#[tauri::command]
async fn install_autostart(_path: String, _token: String, autostart: bool) -> Result<String, String> {
    if !autostart {
        return Ok("건너뜀".into());
    }
    tokio::task::spawn_blocking(|| installer::register_autostart())
        .await
        .map_err(|e| format!("Task error: {}", e))?
}

// ------ Main ------

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_dialog::init())
        .invoke_handler(tauri::generate_handler![
            github_device_code,
            github_poll_token,
            check_copilot,
            get_default_install_path,
            install_check_node,
            install_node,
            install_openclaw,
            install_config,
            install_start_gateway,
            install_autostart,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
