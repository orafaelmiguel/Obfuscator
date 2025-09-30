use anyhow::{Result};
use reqwest::blocking::Client;
use serde::{Deserialize, Serialize};
use std::time::Duration;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoginResponse {
    pub accessToken: String,
    pub licensed: bool,
}

#[derive(Debug)]
pub enum AuthError {
    Http(String),
    Status(u16, String),
    Deserialize(String),
    Network(String),
}

impl From<anyhow::Error> for AuthError {
    fn from(e: anyhow::Error) -> Self {
        AuthError::Network(e.to_string())
    }
}

pub fn login_request(base_url: &str, email: &str, password: &str) -> Result<LoginResponse, AuthError> {
    let client = Client::builder()
        .timeout(Duration::from_secs(15))
        .build()
        .map_err(|e| AuthError::Network(e.to_string()))?;

    let url = format!("{}/auth/login", base_url.trim_end_matches('/'));

    let body = serde_json::json!({
        "email": email,
        "password": password
    });

    let resp = client
        .post(&url)
        .json(&body)
        .send()
        .map_err(|e| AuthError::Network(e.to_string()))?;

    let status = resp.status();
    let text = resp.text().map_err(|e| AuthError::Network(e.to_string()))?;

    if !status.is_success() {
        // try to include body in error for debugging
        return Err(AuthError::Status(status.as_u16(), text));
    }

    // parse json
    let parsed: LoginResponse = serde_json::from_str(&text)
        .map_err(|e| AuthError::Deserialize(e.to_string()))?;

    Ok(parsed)
}