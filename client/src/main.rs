#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    sodiumoxide::init().expect("Failed to init sodium");

    // 1. Расшифровать конфиг
    let key = derive_key_from_ed25519(load_key("keys/identity.ed25519")?);
    let config_json = crypto::decrypt_config("config/config.json.enc", &key)?;
    let mut config: Config = serde_json::from_str(&config_json)?;

    // 2. Обновить из P2P
    if let Some(new) = p2p::dht_store::fetch_config_from_dht("config_v5", &TRUSTED_PUB).await {
        if ooni::is_blocked(&new).await { return Err("Compromised".into()); }
        config = update_from_p2p(new);
    }

    // 3. Выбрать транспорт
    let transport = decide_transport(&config, &ai::probe_network()).await;

    // 4. Запустить имитацию
    match config.mode.as_str() {
        "google-meet" => tokio::spawn(mimic::google_meet::google_meet_traffic()),
        "silent" => tokio::spawn(mimic::silent_mode::youtube_background_traffic()),
        _ => {}
    }

    // 5. GUI
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![connect])
        .run(tauri::generate_context!())
        .expect("error");

    Ok(())
}