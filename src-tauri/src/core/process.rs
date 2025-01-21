use windows::Win32::UI::WindowsAndMessaging::{GetForegroundWindow, GetWindowThreadProcessId};
use windows::Win32::System::ProcessStatus::GetModuleFileNameExW;
use windows::Win32::System::Threading::{OpenProcess, PROCESS_QUERY_INFORMATION, PROCESS_VM_READ};
use std::ffi::OsString;
use std::os::windows::ffi::OsStringExt;

/// 检查DNF游戏窗口是否处于激活状态
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
            process_id
        );
        
        if let Ok(handle) = process_handle {
            // 获取进程的可执行文件路径
            let mut buffer = [0u16; 260];
            let len = GetModuleFileNameExW(
                handle,
                None,
                &mut buffer
            );
            
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