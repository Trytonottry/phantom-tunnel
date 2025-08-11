use reqwest;
use serde::Deserialize;

#[derive(Deserialize)]
struct OoniResult {
    measurement_url: String,
    anomaly: bool,
    blocked: bool,
}

pub async fn is_blocked(domain: &str) -> Result<bool, Box<dyn std::error::Error>> {
    let url = format!(
        "https://api.ooni.io/api/v1/measurements?domain={}&limit=1",
        domain
    );

    let client = reqwest::Client::new();
    let res: serde_json::Value = client.get(&url).send().await?.json().await?;

    if let Some(measurements) = res["results"].as_array() {
        if let Some(meas) = measurements.first() {
            return Ok(meas["anomaly"].as_bool().unwrap_or(false)
                || meas["blocked"].as_bool().unwrap_or(false));
        }
    }

    Ok(false) // нет данных → считаем, что не заблокирован
}