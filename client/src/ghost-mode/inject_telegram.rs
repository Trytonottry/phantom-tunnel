#[cfg(windows)]
use windows::{
    Win32::Foundation::*,
    Win32::System::Threading::*,
    Win32::System::Diagnostics::Debug::*,
    Win32::System::Memory::*,
    Win32::System::LibraryLoader::*,
};

#[cfg(windows)]
pub unsafe fn inject_telegram_dll(dll_path: &str) -> Result<(), String> {
    use std::ffi::CString;

    let process_name = CString::new("Telegram.exe").map_err(|_| "CString failed")?;
    let mut entry: PROCESSENTRY32 = std::mem::zeroed();
    entry.dwSize = std::mem::size_of::<PROCESSENTRY32>() as u32;

    let snapshot = CreateToolhelp32Snapshot(TH32CS_SNAPPROCESS, 0)
        .map_err(|e| format!("Snapshot failed: {:?}", e))?;

    if Process32First(snapshot, &mut entry).is_err() {
        return Err("Process32First failed".to_string());
    }

    let mut pid = 0;
    while Process32Next(snapshot, &mut entry).is_ok() {
        if entry
            .szExeFile
            .iter()
            .position(|&c| c == 0)
            .and_then(|pos| std::str::from_utf8(&entry.szExeFile[..pos]).ok())
            .map(|name| name == "Telegram.exe")
            .unwrap_or(false)
        {
            pid = entry.th32ProcessID;
            break;
        }
    }

    if pid == 0 {
        return Err("Telegram not found".to_string());
    }

    let h_process = OpenProcess(
        PROCESS_ALL_ACCESS,
        false,
        pid,
    ).map_err(|e| format!("OpenProcess failed: {:?}", e))?;

    let dll_cstr = CString::new(dll_path).map_err(|_| "Invalid path")?;
    let alloc = VirtualAllocEx(
        h_process,
        None,
        dll_path.len(),
        MEM_COMMIT | MEM_RESERVE,
        PAGE_READWRITE,
    );

    if alloc.is_null() {
        return Err("VirtualAllocEx failed".to_string());
    }

    WriteProcessMemory(
        h_process,
        alloc,
        dll_cstr.as_ptr() as _,
        dll_path.len(),
        None,
    ).map_err(|e| format!("WriteProcessMemory failed: {:?}", e))?;

    let load_lib = GetProcAddress(
        GetModuleHandleA("kernel32.dll").map_err(|_| "kernel32 not found")?,
        "LoadLibraryA\0".as_ptr() as _,
    );

    let h_thread = CreateRemoteThread(
        h_process,
        None,
        0,
        Some(std::mem::transmute(load_lib)),
        alloc,
        0,
        None,
    );

    if h_thread.is_null() {
        return Err("CreateRemoteThread failed".to_string());
    }

    WaitForSingleObject(h_thread, 5000);
    CloseHandle(h_thread);
    CloseHandle(h_process);

    Ok(())
}