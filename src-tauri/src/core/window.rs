//! 窗口检测模块
//!
//! Windows 平台: 检测 DNF 游戏窗口是否处于活动状态
//! 其他平台: Mock 实现，用于 UI 开发

/// DNF 游戏窗口的类名列表
#[cfg(windows)]
const DNF_WINDOW_CLASSES: &[&str] = &[
    "地下城与勇士",           // 国服
    "Notepad",                // 记事本（测试用）
];

// ============================================================================
// Windows 实现
// ============================================================================

#[cfg(windows)]
mod windows_impl {
    use super::DNF_WINDOW_CLASSES;
    use windows::Win32::Foundation::HWND;
    use windows::Win32::UI::WindowsAndMessaging::{GetClassNameW, GetForegroundWindow};

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

            DNF_WINDOW_CLASSES
                .iter()
                .any(|&dnf_class| class_name.contains(dnf_class))
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
}

// ============================================================================
// Mock 实现（非 Windows 平台，用于 UI 开发）
// ============================================================================

#[cfg(not(windows))]
mod mock_impl {
    use std::sync::atomic::{AtomicBool, Ordering};

    /// Mock: 模拟 DNF 窗口是否活动（默认为 true，方便 UI 测试）
    static MOCK_DNF_ACTIVE: AtomicBool = AtomicBool::new(true);

    /// Mock: 检测当前前台窗口是否是 DNF 游戏
    /// 在非 Windows 平台上，默认返回 true 以方便 UI 开发测试
    pub fn is_dnf_active() -> bool {
        MOCK_DNF_ACTIVE.load(Ordering::SeqCst)
    }

    /// Mock: 获取当前前台窗口的类名
    pub fn get_foreground_class_name() -> String {
        if MOCK_DNF_ACTIVE.load(Ordering::SeqCst) {
            "MockDNFWindow".to_string()
        } else {
            "MockOtherWindow".to_string()
        }
    }

    /// Mock: 设置模拟的 DNF 窗口状态（供测试使用）
    #[allow(dead_code)]
    pub fn set_mock_dnf_active(active: bool) {
        MOCK_DNF_ACTIVE.store(active, Ordering::SeqCst);
    }
}

// ============================================================================
// 导出公共接口
// ============================================================================

#[cfg(windows)]
pub use windows_impl::*;

#[cfg(not(windows))]
pub use mock_impl::*;
