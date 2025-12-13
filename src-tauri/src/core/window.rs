//! 窗口检测模块 - 检测 DNF 游戏窗口是否处于活动状态

use windows::Win32::Foundation::HWND;
use windows::Win32::UI::WindowsAndMessaging::{
    GetForegroundWindow, GetClassNameW,
};

/// DNF 游戏窗口的类名列表
const DNF_WINDOW_CLASSES: &[&str] = &[
    "地下城与勇士",           // 国服
    "Notepad",                // 记事本（测试用）
];

/// 检测当前前台窗口是否是 DNF 游戏
pub fn is_dnf_active() -> bool {
    unsafe {
        let hwnd = GetForegroundWindow();
        if hwnd == HWND::default() {
            return false;
        }

        let mut class_name_buf = [0u16; 256];
        let len = GetClassNameW(hwnd, &mut class_name_buf);
        if len == 0 {
            return false;
        }

        let class_name = String::from_utf16_lossy(&class_name_buf[..len as usize]);

        DNF_WINDOW_CLASSES.iter().any(|&dnf_class| {
            class_name.contains(dnf_class)
        })
    }
}

/// 获取当前前台窗口的类名（调试用）
pub fn get_foreground_class_name() -> String {
    unsafe {
        let hwnd = GetForegroundWindow();
        if hwnd == HWND::default() {
            return String::new();
        }

        let mut class_name_buf = [0u16; 256];
        let len = GetClassNameW(hwnd, &mut class_name_buf);
        if len == 0 {
            return String::new();
        }

        String::from_utf16_lossy(&class_name_buf[..len as usize])
    }
}
