use tauri::Manager;

#[tauri::command]
async fn connect_tunnel(mode: String) -> Result<(), String> {
    match mode.as_str() {
        "p2p" => p2p_dht::discover_relays(...).await.map_err(|e| e.to_string())?,
        "quic" => quic_ech::connect_quic_ech(...).await.map_err(|e| e.to_string())?,
        "cf" => cloudflare::connect_worker(...).await.map_err(|e| e.to_string())?,
        _ => return Err("Unknown mode".into()),
    };
    Ok(())
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![connect_tunnel])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}