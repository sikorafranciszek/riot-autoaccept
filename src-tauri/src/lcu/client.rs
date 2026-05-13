use anyhow::{Context, Result};
use base64::{engine::general_purpose::STANDARD as B64, Engine as _};
use reqwest::{Client, Method, StatusCode};
use serde::de::DeserializeOwned;
use std::time::Duration;

/// Lightweight HTTPS client for the League Client Update API.
///
/// The LCU serves a self-signed certificate on localhost, so we explicitly
/// disable cert verification — there is no risk of MITM since we only ever
/// talk to `127.0.0.1`.
#[derive(Clone)]
pub struct LcuClient {
    http: Client,
    base: String,
    auth: String,
}

impl LcuClient {
    pub fn new(port: u16, token: &str) -> Result<Self> {
        let http = Client::builder()
            .danger_accept_invalid_certs(true)
            .timeout(Duration::from_secs(4))
            .build()
            .context("build reqwest client")?;

        let auth = format!("Basic {}", B64.encode(format!("riot:{}", token)));

        Ok(Self {
            http,
            base: format!("https://127.0.0.1:{}", port),
            auth,
        })
    }

    pub fn auth_header(&self) -> &str {
        &self.auth
    }

    async fn request_raw(&self, method: Method, path: &str) -> Result<(StatusCode, String)> {
        let url = format!("{}{}", self.base, path);
        let res = self
            .http
            .request(method, &url)
            .header("Authorization", &self.auth)
            .header("Content-Type", "application/json")
            .send()
            .await
            .with_context(|| format!("LCU request {}", url))?;
        let status = res.status();
        let body = res.text().await.unwrap_or_default();
        Ok((status, body))
    }

    pub async fn get<T: DeserializeOwned>(&self, path: &str) -> Result<T> {
        let (status, body) = self.request_raw(Method::GET, path).await?;
        anyhow::ensure!(status.is_success(), "GET {} -> {}", path, status);
        serde_json::from_str(&body).with_context(|| format!("decode {}", path))
    }

    pub async fn get_text(&self, path: &str) -> Result<String> {
        let (status, body) = self.request_raw(Method::GET, path).await?;
        anyhow::ensure!(status.is_success(), "GET {} -> {}", path, status);
        // gameflow-phase returns a quoted string e.g. "\"Matchmaking\""
        Ok(body.trim().trim_matches('"').to_owned())
    }

    pub async fn post_empty(&self, path: &str) -> Result<StatusCode> {
        let (status, _body) = self.request_raw(Method::POST, path).await?;
        Ok(status)
    }
}
