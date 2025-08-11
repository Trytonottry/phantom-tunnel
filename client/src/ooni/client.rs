pub async fn is_blocked(domain: &str) -> bool {
    reqwest::get(format!("https://api.ooni.io/api/v1/measurements?domain={}&limit=1", domain))
        .await
        .map(|r| r.json().await.unwrap_or_default())
        .map(|j| j["anomaly"].as_bool().unwrap_or(false))
        .unwrap_or(false)
}