//! 核心功能模块
//!
//! 提供平台无关的抽象接口和平台特定实现

pub mod autofire;
pub mod keyboard;
pub mod traits;
pub mod window;

// 重新导出常用类型（供外部使用）
#[allow(unused_imports)]
pub use keyboard::vk;
#[allow(unused_imports)]
pub use keyboard::DefaultKeyboardDriver;
#[allow(unused_imports)]
pub use traits::{
    HookResult, KeyEvent, KeyEventType, KeyHookCallback, KeyboardDriver, KeyboardHook,
    WindowDetector,
};
#[allow(unused_imports)]
pub use window::DefaultWindowDetector;
