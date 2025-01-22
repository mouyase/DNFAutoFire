#[cfg(target_os = "windows")]
use std::ffi::OsString;
#[cfg(target_os = "windows")]
use std::os::windows::ffi::OsStringExt;
#[cfg(target_os = "windows")]
use windows::Win32::System::ProcessStatus::GetModuleFileNameExW;
#[cfg(target_os = "windows")]
use windows::Win32::System::Threading::{OpenProcess, PROCESS_QUERY_INFORMATION, PROCESS_VM_READ};
#[cfg(target_os = "windows")]
use windows::Win32::UI::WindowsAndMessaging::{GetForegroundWindow, GetWindowThreadProcessId};

/// 检查DNF游戏窗口是否处于激活状态
#[cfg(target_os = "windows")]
pub fn is_game_window_active() -> bool {
    unsafe {
        // 获取当前激活窗口的句柄
        let hwnd = GetForegroundWindow();
        if hwnd.0 == 0 {
            return false;
        }

        // 获取窗口对应的进程ID
        let mut process_id: u32 = 0;
        GetWindowThreadProcessId(hwnd, Some(&mut process_id));
        if process_id == 0 {
            return false;
        }

        // 打开进程
        let process_handle = OpenProcess(
            PROCESS_QUERY_INFORMATION | PROCESS_VM_READ,
            false,
            process_id,
        );

        if let Ok(handle) = process_handle {
            // 获取进程的可执行文件路径
            let mut buffer = [0u16; 260];
            let len = GetModuleFileNameExW(handle, None, &mut buffer);

            if len > 0 {
                let process_path = OsString::from_wide(&buffer[..len as usize])
                    .to_string_lossy()
                    .to_lowercase();

                return process_path.contains("dnf.exe");
            }
        }

        false
    }
}

// macOS 版本的实现
#[cfg(target_os = "macos")]
pub fn is_game_window_active() -> bool {
    // 在 macOS 上暂时返回 true，方便开发调试
    println!("Mac 环境模拟窗口检查");
    true
}

// 其他系统的实现
#[cfg(not(any(target_os = "windows", target_os = "macos")))]
pub fn is_game_window_active() -> bool {
    println!("当前系统暂不支持窗口检查");
    false
}
