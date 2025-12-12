//! 键盘布局定义

use crate::core::keyboard::vk;

/// 按键信息
pub struct KeyInfo {
    /// 显示标签
    pub label: String,
    /// 虚拟键码
    pub vk: u16,
    /// 按键宽度
    pub width: f32,
}

impl KeyInfo {
    pub fn new(label: &str, vk: u16) -> Self {
        Self {
            label: label.to_string(),
            vk,
            width: 36.0, // 默认宽度
        }
    }

    pub fn with_width(label: &str, vk: u16, width: f32) -> Self {
        Self {
            label: label.to_string(),
            vk,
            width,
        }
    }
}

/// 键盘布局（类似实际键盘）
pub static KEYBOARD_LAYOUT: once_cell::sync::Lazy<Vec<Vec<KeyInfo>>> = once_cell::sync::Lazy::new(|| {
    vec![
        // 第一行: 数字行
        vec![
            KeyInfo::new("Esc", vk::VK_ESCAPE),
            KeyInfo::new("1", vk::VK_1),
            KeyInfo::new("2", vk::VK_2),
            KeyInfo::new("3", vk::VK_3),
            KeyInfo::new("4", vk::VK_4),
            KeyInfo::new("5", vk::VK_5),
            KeyInfo::new("6", vk::VK_6),
            KeyInfo::new("7", vk::VK_7),
            KeyInfo::new("8", vk::VK_8),
            KeyInfo::new("9", vk::VK_9),
            KeyInfo::new("0", vk::VK_0),
            KeyInfo::with_width("Back", vk::VK_BACK, 48.0),
        ],
        // 第二行: QWERTY
        vec![
            KeyInfo::with_width("Tab", vk::VK_TAB, 48.0),
            KeyInfo::new("Q", vk::VK_Q),
            KeyInfo::new("W", vk::VK_W),
            KeyInfo::new("E", vk::VK_E),
            KeyInfo::new("R", vk::VK_R),
            KeyInfo::new("T", vk::VK_T),
            KeyInfo::new("Y", vk::VK_Y),
            KeyInfo::new("U", vk::VK_U),
            KeyInfo::new("I", vk::VK_I),
            KeyInfo::new("O", vk::VK_O),
            KeyInfo::new("P", vk::VK_P),
        ],
        // 第三行: ASDF
        vec![
            KeyInfo::new("A", vk::VK_A),
            KeyInfo::new("S", vk::VK_S),
            KeyInfo::new("D", vk::VK_D),
            KeyInfo::new("F", vk::VK_F),
            KeyInfo::new("G", vk::VK_G),
            KeyInfo::new("H", vk::VK_H),
            KeyInfo::new("J", vk::VK_J),
            KeyInfo::new("K", vk::VK_K),
            KeyInfo::new("L", vk::VK_L),
            KeyInfo::with_width("Enter", vk::VK_RETURN, 60.0),
        ],
        // 第四行: ZXCV
        vec![
            KeyInfo::with_width("Shift", vk::VK_SHIFT, 60.0),
            KeyInfo::new("Z", vk::VK_Z),
            KeyInfo::new("X", vk::VK_X),
            KeyInfo::new("C", vk::VK_C),
            KeyInfo::new("V", vk::VK_V),
            KeyInfo::new("B", vk::VK_B),
            KeyInfo::new("N", vk::VK_N),
            KeyInfo::new("M", vk::VK_M),
        ],
        // 第五行: 空格和方向键
        vec![
            KeyInfo::with_width("Ctrl", vk::VK_CONTROL, 48.0),
            KeyInfo::with_width("Alt", vk::VK_MENU, 48.0),
            KeyInfo::with_width("Space", vk::VK_SPACE, 200.0),
            KeyInfo::new("Left", vk::VK_LEFT),
            KeyInfo::new("Up", vk::VK_UP),
            KeyInfo::new("Down", vk::VK_DOWN),
            KeyInfo::new("Right", vk::VK_RIGHT),
        ],
        // 第六行: 功能键
        vec![
            KeyInfo::new("F1", vk::VK_F1),
            KeyInfo::new("F2", vk::VK_F2),
            KeyInfo::new("F3", vk::VK_F3),
            KeyInfo::new("F4", vk::VK_F4),
            KeyInfo::new("F5", vk::VK_F5),
            KeyInfo::new("F6", vk::VK_F6),
            KeyInfo::new("F7", vk::VK_F7),
            KeyInfo::new("F8", vk::VK_F8),
            KeyInfo::new("F9", vk::VK_F9),
            KeyInfo::new("F10", vk::VK_F10),
            KeyInfo::new("F11", vk::VK_F11),
            KeyInfo::new("F12", vk::VK_F12),
        ],
        // 第七行: 小键盘
        vec![
            KeyInfo::new("Num0", vk::VK_NUMPAD0),
            KeyInfo::new("Num1", vk::VK_NUMPAD1),
            KeyInfo::new("Num2", vk::VK_NUMPAD2),
            KeyInfo::new("Num3", vk::VK_NUMPAD3),
            KeyInfo::new("Num4", vk::VK_NUMPAD4),
            KeyInfo::new("Num5", vk::VK_NUMPAD5),
            KeyInfo::new("Num6", vk::VK_NUMPAD6),
            KeyInfo::new("Num7", vk::VK_NUMPAD7),
            KeyInfo::new("Num8", vk::VK_NUMPAD8),
            KeyInfo::new("Num9", vk::VK_NUMPAD9),
        ],
    ]
});
