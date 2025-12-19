//! 窗口检测模块
//!
//! 检测目标窗口（DNF 游戏或记事本）是否处于活动状态

/// DNF 游戏窗口的类名列表
#[cfg(windows)]
const TARGET_WINDOW_CLASSES: &[&str] = &[
    "地下城与勇士", // DNF 国服
    "Notepad",      // 记事本（测试用）
];

/// 窗口检测器接口
pub trait WindowDetector: Send + Sync + std::fmt::Debug {
    /// 检测目标窗口是否处于活动状态
    fn is_target_active(&self) -> bool;

    /// 获取当前前台窗口的类名
    fn get_foreground_class_name(&self) -> String;
}

// ============================================================================
// Windows 实现
// ============================================================================

#[cfg(windows)]
mod windows_impl {
    use super::*;
    use windows::Win32::Foundation::HWND;
    use windows::Win32::UI::WindowsAndMessaging::{GetClassNameW, GetForegroundWindow};

    /// Windows 窗口检测器
    #[derive(Debug, Default)]
    pub struct WindowsWindowDetector;

    impl WindowsWindowDetector {
        pub fn new() -> Self {
            Self
        }
    }

    impl WindowDetector for WindowsWindowDetector {
        fn is_target_active(&self) -> bool {
            let class_name = self.get_foreground_class_name();
            if class_name.is_empty() {
                return false;
            }

            let is_match = TARGET_WINDOW_CLASSES
                .iter()
                .any(|target| class_name.contains(target));

            // 调试日志
            static LAST_CLASS: std::sync::Mutex<String> = std::sync::Mutex::new(String::new());
            if let Ok(mut last) = LAST_CLASS.lock() {
                if *last != class_name {
                    println!(
                        "[WindowDetector] 窗口: '{}' | 匹配: {}",
                        class_name, is_match
                    );
                    println!("[WindowDetector] 目标列表: {:?}", TARGET_WINDOW_CLASSES);
                    *last = class_name.clone();
                }
            }

            is_match
        }

        fn get_foreground_class_name(&self) -> String {
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
}

#[cfg(windows)]
pub use windows_impl::WindowsWindowDetector;
