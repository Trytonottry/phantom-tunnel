use reqwest;
use serde::{Deserialize, Serialize};
use std::fs;
use tokio::time::{interval, Duration};

#[derive(Serialize, Deserialize, Clone)]
pub struct PhantomConfig {
    pub relays: Vec<String>,
    pub obfuscation_profile: String,
    pub transport: String, // "p2p", "quic", "cf"
    pub silent_mode: bool,
    pub ai_enabled: bool,
}

pub async fn start_auto_update(
    url: &str,
    interval_sec: u64,
) -> Result<PhantomConfig, Box<dyn std::error::Error>> {
    let client = reqwest::Client::new();
    let mut interval = interval(Duration::from_secs(interval_sec));

    loop {
        interval.tick().await;

        let res = client
            .get(url)
            .header("User-Agent", "Phantom-Tunnel/v4")
            .send()
            .await;

        if let Ok(resp) = res {
            if resp.status().is_success() {
                let config: PhantomConfig = resp.json().await?;
                fs::write("config/config.json", serde_json::to_string_pretty(&config)?)?;
                log::info!("Конфиг обновлён: {}", config.transport);
                return Ok(config);
            }
        }
    }
}