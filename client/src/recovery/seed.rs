use bip39::{Mnemonic, Seed};
use hmac_sha512::HmacSha512;
use chacha20::cipher::KeyInit;

#[tauri::command]
pub fn recover_from_seed(seed_phrase: String) -> Result<String, String> {
    let mnemonic = Mnemonic::from_phrase(&seed_phrase, Language::English)
        .map_err(|e| e.to_string())?;

    let seed = Seed::from(&mnemonic);
    let key = &seed.as_bytes()[..32];

    // Восстановить конфиг, ключи, узлы
    let config = reconstruct_config_from_seed(key);
    save_config(&config);

    Ok("Восстановлено".to_string())
}