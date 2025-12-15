//! 窗口检测模块
//!
//! Windows 平台: 检测 DNF 游戏窗口是否处于活动状态
//! 其他平台: Mock 实现，用于 UI 开发和测试

use super::traits::WindowDetector;

/// DNF 游戏窗口的类名列表
#[cfg(windows)]
const DNF_WINDOW_CLASSES: &[&str] = &[
    "地下城与勇士", // 国服
    "Notepad",      // 记事本（测试用）
];

// ============================================================================
// Windows 实现
// ============================================================================

#[cfg(windows)]
mod windows_impl {
    use super::*;
    use windows::Win32::Foundation::HWND;
    use windows::Win32::UI::WindowsAndMessaging::{GetClassNameW, GetForegroundWindow};

    /// Windows 窗口检测器实现
    #[derive(Debug, Default)]
    pub struct WindowsWindowDetector {
        /// 目标窗口类名列表
        target_classes: Vec<String>,
    }

    impl WindowsWindowDetector {
        /// 创建默认的 DNF 窗口检测器
        pub fn new() -> Self {
            Self {
                target_classes: DNF_WINDOW_CLASSES.iter().map(|s| s.to_string()).collect(),
            }
        }

        /// 创建自定义目标窗口检测器
        pub fn with_targets(target_classes: Vec<String>) -> Self {
            Self { target_classes }
        }
    }

    impl WindowDetector for WindowsWindowDetector {
        fn is_target_active(&self) -> bool {
            let class_name = self.get_foreground_class_name();
            if class_name.is_empty() {
                return false;
            }

            self.target_classes
                .iter()
                .any(|target| class_name.contains(target))
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

// ============================================================================
// Mock 实现（用于非 Windows 平台和测试）
// ============================================================================

#[allow(dead_code)]
pub mod mock {
    use super::*;
    use parking_lot::RwLock;
    use std::sync::Arc;

    /// Mock 窗口检测器，用于测试
    #[derive(Debug, Clone)]
    pub struct MockWindowDetector {
        /// 模拟的活动状态
        is_active: Arc<RwLock<bool>>,
        /// 模拟的窗口类名
        class_name: Arc<RwLock<String>>,
    }

    impl Default for MockWindowDetector {
        fn default() -> Self {
            Self::new()
        }
    }

    impl MockWindowDetector {
        /// 创建 Mock 检测器，默认目标窗口处于活动状态
        pub fn new() -> Self {
            Self {
                is_active: Arc::new(RwLock::new(true)),
                class_name: Arc::new(RwLock::new("MockDNFWindow".to_string())),
            }
        }

        /// 创建指定初始状态的 Mock 检测器
        pub fn with_state(is_active: bool, class_name: &str) -> Self {
            Self {
                is_active: Arc::new(RwLock::new(is_active)),
                class_name: Arc::new(RwLock::new(class_name.to_string())),
            }
        }

        /// 设置模拟的活动状态
        pub fn set_active(&self, active: bool) {
            *self.is_active.write() = active;
        }

        /// 设置模拟的窗口类名
        pub fn set_class_name(&self, name: &str) {
            *self.class_name.write() = name.to_string();
        }

        /// 获取当前模拟的活动状态
        pub fn get_active(&self) -> bool {
            *self.is_active.read()
        }
    }

    impl WindowDetector for MockWindowDetector {
        fn is_target_active(&self) -> bool {
            *self.is_active.read()
        }

        fn get_foreground_class_name(&self) -> String {
            self.class_name.read().clone()
        }
    }

    /// 总是返回活动状态的检测器（简化版，用于测试）
    #[derive(Debug, Default, Clone, Copy)]
    pub struct AlwaysActiveDetector;

    impl WindowDetector for AlwaysActiveDetector {
        fn is_target_active(&self) -> bool {
            true
        }

        fn get_foreground_class_name(&self) -> String {
            "AlwaysActive".to_string()
        }
    }

    /// 总是返回非活动状态的检测器
    #[derive(Debug, Default, Clone, Copy)]
    pub struct NeverActiveDetector;

    impl WindowDetector for NeverActiveDetector {
        fn is_target_active(&self) -> bool {
            false
        }

        fn get_foreground_class_name(&self) -> String {
            "NeverActive".to_string()
        }
    }
}

// ============================================================================
// 导出
// ============================================================================

#[cfg(windows)]
pub use windows_impl::WindowsWindowDetector;

#[cfg(not(windows))]
pub use mock::MockWindowDetector as DefaultWindowDetector;

#[cfg(windows)]
pub use windows_impl::WindowsWindowDetector as DefaultWindowDetector;

// ============================================================================
// 单元测试
// ============================================================================

#[cfg(test)]
mod tests {
    use super::mock::*;
    use super::*;

    #[test]
    fn test_mock_detector_default_active() {
        let detector = MockWindowDetector::new();
        assert!(detector.is_target_active());
        assert_eq!(detector.get_foreground_class_name(), "MockDNFWindow");
    }

    #[test]
    fn test_mock_detector_set_active() {
        let detector = MockWindowDetector::new();

        detector.set_active(false);
        assert!(!detector.is_target_active());

        detector.set_active(true);
        assert!(detector.is_target_active());
    }

    #[test]
    fn test_mock_detector_set_class_name() {
        let detector = MockWindowDetector::new();

        detector.set_class_name("CustomWindow");
        assert_eq!(detector.get_foreground_class_name(), "CustomWindow");
    }

    #[test]
    fn test_mock_detector_with_state() {
        let detector = MockWindowDetector::with_state(false, "TestWindow");
        assert!(!detector.is_target_active());
        assert_eq!(detector.get_foreground_class_name(), "TestWindow");
    }

    #[test]
    fn test_always_active_detector() {
        let detector = AlwaysActiveDetector;
        assert!(detector.is_target_active());
    }

    #[test]
    fn test_never_active_detector() {
        let detector = NeverActiveDetector;
        assert!(!detector.is_target_active());
    }
}
