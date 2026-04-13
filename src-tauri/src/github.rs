// ============================================================
// GitHub OAuth — Device Flow
// ============================================================

use serde::{Deserialize, Serialize};
use std::time::Duration;

const GITHUB_CLIENT_ID: &str = "YOUR_GITHUB_OAUTH_APP_CLIENT_ID"; // TODO: Replace after creating OAuth App

#[derive(Debug, Serialize, Deserialize)]
pub struct DeviceCodeResponse {
    pub device_code: String,
    pub user_code: String,
    pub verification_uri: String,
    pub expires_in: u64,
    pub interval: u64,
}

#[derive(Debug, Deserialize)]
struct TokenResponse {
    access_token: Option<String>,
    error: Option<String>,
}

/// Request a device code from GitHub
pub async fn request_device_code() -> Result<DeviceCodeResponse, String> {
    let client = reqwest::Client::new();
    let resp = client
        .post("https://github.com/login/device/code")
        .header("Accept", "application/json")
        .form(&[
            ("client_id", GITHUB_CLIENT_ID),
            ("scope", "read:user read:org"),
        ])
        .send()
        .await
        .map_err(|e| format!("Device code request failed: {}", e))?;

    resp.json::<DeviceCodeResponse>()
        .await
        .map_err(|e| format!("Failed to parse device code response: {}", e))
}

/// Poll GitHub for the access token after user authorizes
pub async fn poll_for_token(device_code: &str, interval: u64) -> Result<String, String> {
    let client = reqwest::Client::new();
    let interval_duration = Duration::from_secs(interval.max(5));

    loop {
        tokio::time::sleep(interval_duration).await;

        let resp = client
            .post("https://github.com/login/oauth/access_token")
            .header("Accept", "application/json")
            .form(&[
                ("client_id", GITHUB_CLIENT_ID),
                ("device_code", device_code),
                ("grant_type", "urn:ietf:params:oauth:grant-type:device_code"),
            ])
            .send()
            .await
            .map_err(|e| format!("Token poll failed: {}", e))?;

        let token_resp: TokenResponse = resp
            .json()
            .await
            .map_err(|e| format!("Failed to parse token response: {}", e))?;

        if let Some(token) = token_resp.access_token {
            return Ok(token);
        }

        match token_resp.error.as_deref() {
            Some("authorization_pending") => continue,
            Some("slow_down") => {
                tokio::time::sleep(Duration::from_secs(5)).await;
                continue;
            }
            Some("expired_token") => return Err("인증 시간이 만료되었습니다. 다시 시도해주세요.".into()),
            Some("access_denied") => return Err("인증이 거부되었습니다.".into()),
            Some(err) => return Err(format!("OAuth error: {}", err)),
            None => continue,
        }
    }
}

/// Check if the user has an active Copilot subscription
pub async fn check_copilot_subscription(token: &str) -> Result<bool, String> {
    let client = reqwest::Client::new();
    
    // Check Copilot access via the user's Copilot endpoint
    let resp = client
        .get("https://api.github.com/copilot_billing/seats")
        .header("Authorization", format!("Bearer {}", token))
        .header("User-Agent", "auto-install-openclaw")
        .header("Accept", "application/vnd.github+json")
        .send()
        .await
        .map_err(|e| format!("Copilot check failed: {}", e))?;

    // If we can access copilot endpoints, user likely has a subscription
    // Alternative: check user's plan or features
    match resp.status().as_u16() {
        200 => Ok(true),
        401 | 403 | 404 => {
            // Try alternative endpoint — individual Copilot subscription
            let resp2 = client
                .get("https://api.github.com/user")
                .header("Authorization", format!("Bearer {}", token))
                .header("User-Agent", "auto-install-openclaw")
                .send()
                .await
                .map_err(|e| format!("User check failed: {}", e))?;
            
            if resp2.status().is_success() {
                // User is authenticated but may not have copilot
                // TODO: Find the exact endpoint to check individual Copilot subscription
                // For now, we assume they need to verify manually
                Ok(false)
            } else {
                Err("GitHub 인증에 실패했습니다.".into())
            }
        }
        _ => Err(format!("Unexpected response: {}", resp.status())),
    }
}
