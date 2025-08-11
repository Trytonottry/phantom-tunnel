#[cfg(target_os = "windows")]
pub fn inject_into_process(process_name: &str) -> Result<(), Box<dyn std::error::Error>> {
    use windows::Win32::System::Threading::*;
    use windows::Win32::System::Diagnostics::Debug::*;

    // Найти процесс
    let snapshot = CreateToolhelp32Snapshot(TH32CS_SNAPPROCESS, 0)?;
    // Найти PID
    // Выделить память
    // Записать DLL
    // Создать поток
    Ok(())
}