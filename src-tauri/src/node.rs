// ============================================================
// Node.js Installation Manager
// ============================================================

use std::process::Command;
use std::path::PathBuf;

const NODE_LTS_VERSION: &str = "20.18.0";
const NODE_DOWNLOAD_URL: &str = "https://nodejs.org/dist/v20.18.0/node-v20.18.0-x64.msi";

/// Check if Node.js is installed and return version
pub fn check_node() -> Result<Option<String>, String> {
    match Command::new("node").arg("--version").output() {
        Ok(output) => {
            if output.status.success() {
                let version = String::from_utf8_lossy(&output.stdout).trim().to_string();
                Ok(Some(version))
            } else {
                Ok(None)
            }
        }
        Err(_) => Ok(None),
    }
}

/// Download and install Node.js silently
pub async fn install_node() -> Result<String, String> {
    // Check if already installed
    if let Ok(Some(version)) = check_node() {
        return Ok(format!("Node.js {} 이미 설치됨", version));
    }

    // Download Node.js MSI
    let temp_dir = std::env::temp_dir();
    let msi_path = temp_dir.join(format!("node-v{}-x64.msi", NODE_LTS_VERSION));

    let client = reqwest::Client::new();
    let resp = client
        .get(NODE_DOWNLOAD_URL)
        .send()
        .await
        .map_err(|e| format!("Node.js 다운로드 실패: {}", e))?;

    let bytes = resp
        .bytes()
        .await
        .map_err(|e| format!("Node.js 다운로드 실패: {}", e))?;

    std::fs::write(&msi_path, bytes)
        .map_err(|e| format!("파일 저장 실패: {}", e))?;

    // Silent install
    let output = Command::new("msiexec")
        .args(&["/i", &msi_path.to_string_lossy(), "/quiet", "/norestart"])
        .output()
        .map_err(|e| format!("Node.js 설치 실행 실패: {}", e))?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        return Err(format!("Node.js 설치 실패: {}", stderr));
    }

    // Cleanup
    let _ = std::fs::remove_file(&msi_path);

    Ok(format!("Node.js v{} 설치 완료", NODE_LTS_VERSION))
}

/// Get the npm command path
pub fn get_npm_path() -> PathBuf {
    // After fresh install, npm might be in Program Files
    let program_files = std::env::var("ProgramFiles").unwrap_or_else(|_| "C:\\Program Files".into());
    let npm_path = PathBuf::from(&program_files).join("nodejs").join("npm.cmd");
    
    if npm_path.exists() {
        npm_path
    } else {
        PathBuf::from("npm")
    }
}
