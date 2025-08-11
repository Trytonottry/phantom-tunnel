use std::fs;
use zeroize::Zeroize;

pub fn self_destruct() -> ! {
    // 1. Стереть ключи
    secure_wipe("keys/identity.ed25519");
    secure_wipe("config/config.json.enc");

    // 2. Стереть временные файлы
    for entry in fs::read_dir("/tmp").unwrap() {
        let path = entry.unwrap().path();
        if path.to_string_lossy().contains("phantom") {
            let mut data = fs::read(&path).unwrap_or_default();
            data.zeroize();
            fs::write(&path, &data).ok();
            fs::remove_file(&path).ok();
        }
    }

    // 3. Перезаписать память
    let mut noise = vec![0u8; 1024 * 1024];
    rand::thread_rng().fill(&mut noise[..]);
    noise.zeroize();

    // 4. Закрыть процесс
    std::process::exit(0);
}

fn secure_wipe(path: &str) {
    if let Ok(mut data) = fs::read(path) {
        data.zeroize();
        let _ = fs::write(path, &data);
        let _ = fs::remove_file(path);
    }
}

// Триггеры
pub fn check_hostile_environment() -> bool {
    // Проверка: запущен ли Wireshark?
    is_process_running("wireshark.exe")
        || is_process_running("volatility.exe")
        || is_on_fsb_network()  // по SSID
        || is_debugger_attached()
}

#[cfg(windows)]
fn is_debugger_attached() -> bool {
    use windows::Win32::System::Diagnostics::Debug::IsDebuggerPresent;
    unsafe { IsDebuggerPresent() }.as_bool()
}