//! 键盘输入模块
//!
//! 提供 Windows SendInput API 封装，用于发送按键到游戏

use std::fmt::Debug;

/// 常用按键的虚拟键码（完整104键）
#[allow(dead_code)]
pub mod vk {
    // 字母键 A-Z
    pub const VK_A: u16 = 0x41;
    pub const VK_B: u16 = 0x42;
    pub const VK_C: u16 = 0x43;
    pub const VK_D: u16 = 0x44;
    pub const VK_E: u16 = 0x45;
    pub const VK_F: u16 = 0x46;
    pub const VK_G: u16 = 0x47;
    pub const VK_H: u16 = 0x48;
    pub const VK_I: u16 = 0x49;
    pub const VK_J: u16 = 0x4A;
    pub const VK_K: u16 = 0x4B;
    pub const VK_L: u16 = 0x4C;
    pub const VK_M: u16 = 0x4D;
    pub const VK_N: u16 = 0x4E;
    pub const VK_O: u16 = 0x4F;
    pub const VK_P: u16 = 0x50;
    pub const VK_Q: u16 = 0x51;
    pub const VK_R: u16 = 0x52;
    pub const VK_S: u16 = 0x53;
    pub const VK_T: u16 = 0x54;
    pub const VK_U: u16 = 0x55;
    pub const VK_V: u16 = 0x56;
    pub const VK_W: u16 = 0x57;
    pub const VK_X: u16 = 0x58;
    pub const VK_Y: u16 = 0x59;
    pub const VK_Z: u16 = 0x5A;

    // 数字键 0-9
    pub const VK_0: u16 = 0x30;
    pub const VK_1: u16 = 0x31;
    pub const VK_2: u16 = 0x32;
    pub const VK_3: u16 = 0x33;
    pub const VK_4: u16 = 0x34;
    pub const VK_5: u16 = 0x35;
    pub const VK_6: u16 = 0x36;
    pub const VK_7: u16 = 0x37;
    pub const VK_8: u16 = 0x38;
    pub const VK_9: u16 = 0x39;

    // 功能键 F1-F12
    pub const VK_F1: u16 = 0x70;
    pub const VK_F2: u16 = 0x71;
    pub const VK_F3: u16 = 0x72;
    pub const VK_F4: u16 = 0x73;
    pub const VK_F5: u16 = 0x74;
    pub const VK_F6: u16 = 0x75;
    pub const VK_F7: u16 = 0x76;
    pub const VK_F8: u16 = 0x77;
    pub const VK_F9: u16 = 0x78;
    pub const VK_F10: u16 = 0x79;
    pub const VK_F11: u16 = 0x7A;
    pub const VK_F12: u16 = 0x7B;

    // 控制键
    pub const VK_ESCAPE: u16 = 0x1B;
    pub const VK_TAB: u16 = 0x09;
    pub const VK_CAPITAL: u16 = 0x14; // Caps Lock
    pub const VK_LSHIFT: u16 = 0xA0;
    pub const VK_RSHIFT: u16 = 0xA1;
    pub const VK_LCONTROL: u16 = 0xA2;
    pub const VK_RCONTROL: u16 = 0xA3;
    pub const VK_LMENU: u16 = 0xA4; // Left Alt
    pub const VK_RMENU: u16 = 0xA5; // Right Alt
    pub const VK_LWIN: u16 = 0x5B; // Left Windows
    pub const VK_RWIN: u16 = 0x5C; // Right Windows
    pub const VK_APPS: u16 = 0x5D; // Menu key

    // 编辑键
    pub const VK_BACK: u16 = 0x08; // Backspace
    pub const VK_RETURN: u16 = 0x0D; // Enter
    pub const VK_SPACE: u16 = 0x20;
    pub const VK_INSERT: u16 = 0x2D;
    pub const VK_DELETE: u16 = 0x2E;
    pub const VK_HOME: u16 = 0x24;
    pub const VK_END: u16 = 0x23;
    pub const VK_PRIOR: u16 = 0x21; // Page Up
    pub const VK_NEXT: u16 = 0x22; // Page Down

    // 方向键
    pub const VK_LEFT: u16 = 0x25;
    pub const VK_UP: u16 = 0x26;
    pub const VK_RIGHT: u16 = 0x27;
    pub const VK_DOWN: u16 = 0x28;

    // 系统键
    pub const VK_SNAPSHOT: u16 = 0x2C; // Print Screen
    pub const VK_SCROLL: u16 = 0x91; // Scroll Lock
    pub const VK_PAUSE: u16 = 0x13;

    // 小键盘
    pub const VK_NUMLOCK: u16 = 0x90;
    pub const VK_NUMPAD0: u16 = 0x60;
    pub const VK_NUMPAD1: u16 = 0x61;
    pub const VK_NUMPAD2: u16 = 0x62;
    pub const VK_NUMPAD3: u16 = 0x63;
    pub const VK_NUMPAD4: u16 = 0x64;
    pub const VK_NUMPAD5: u16 = 0x65;
    pub const VK_NUMPAD6: u16 = 0x66;
    pub const VK_NUMPAD7: u16 = 0x67;
    pub const VK_NUMPAD8: u16 = 0x68;
    pub const VK_NUMPAD9: u16 = 0x69;
    pub const VK_MULTIPLY: u16 = 0x6A; // *
    pub const VK_ADD: u16 = 0x6B; // +
    pub const VK_SUBTRACT: u16 = 0x6D; // -
    pub const VK_DECIMAL: u16 = 0x6E; // .
    pub const VK_DIVIDE: u16 = 0x6F; // /

    // OEM 按键（特殊符号）
    pub const VK_OEM_1: u16 = 0xBA; // ;:
    pub const VK_OEM_PLUS: u16 = 0xBB; // =+
    pub const VK_OEM_COMMA: u16 = 0xBC; // ,<
    pub const VK_OEM_MINUS: u16 = 0xBD; // -_
    pub const VK_OEM_PERIOD: u16 = 0xBE; // .>
    pub const VK_OEM_2: u16 = 0xBF; // /?
    pub const VK_OEM_3: u16 = 0xC0; // `~
    pub const VK_OEM_4: u16 = 0xDB; // [{
    pub const VK_OEM_5: u16 = 0xDC; // \|
    pub const VK_OEM_6: u16 = 0xDD; // ]}
    pub const VK_OEM_7: u16 = 0xDE; // '"

    // 旧版常量（保持向后兼容）
    pub const VK_SHIFT: u16 = 0x10;
    pub const VK_CONTROL: u16 = 0x11;
    pub const VK_MENU: u16 = 0x12; // Alt
}

/// 键盘驱动核心接口
pub trait KeyboardDriver: Send + Sync + Debug {
    /// 虚拟键码转扫描码
    fn vk_to_scan_code(&self, vk: u16) -> u16;

    /// 发送按键按下
    fn send_key_down(&self, vk: u16, scan_code: u16);

    /// 发送按键释放
    fn send_key_up(&self, vk: u16, scan_code: u16);

    /// 发送完整按键（按下+释放）
    #[allow(dead_code)]
    fn send_key_press(&self, vk: u16) {
        let sc = self.vk_to_scan_code(vk);
        self.send_key_down(vk, sc);
        self.send_key_up(vk, sc);
    }

    /// 发送游戏专用按键按下（仅游戏识别，聊天框不识别）
    fn send_game_key_down(&self, scan_code: u16);

    /// 发送游戏专用按键释放
    fn send_game_key_up(&self, scan_code: u16);

    /// 检测按键是否被物理按下
    #[allow(dead_code)]
    fn is_key_pressed(&self, vk: u16) -> bool;
}

// ============================================================================
// Windows 实现
// ============================================================================

#[cfg(windows)]
mod windows_impl {
    use super::*;
    use windows::Win32::UI::Input::KeyboardAndMouse::{
        GetAsyncKeyState, MapVirtualKeyW, SendInput, INPUT, INPUT_0, INPUT_KEYBOARD, KEYBDINPUT,
        KEYBD_EVENT_FLAGS, KEYEVENTF_KEYUP, KEYEVENTF_SCANCODE, MAPVK_VK_TO_VSC, VIRTUAL_KEY,
    };

    /// Windows 键盘驱动实现
    #[derive(Debug, Default)]
    pub struct WindowsKeyboardDriver;

    impl WindowsKeyboardDriver {
        pub fn new() -> Self {
            Self
        }
    }

    impl KeyboardDriver for WindowsKeyboardDriver {
        fn vk_to_scan_code(&self, vk: u16) -> u16 {
            unsafe { MapVirtualKeyW(vk as u32, MAPVK_VK_TO_VSC) as u16 }
        }

        fn send_key_down(&self, vk: u16, scan_code: u16) {
            let input = INPUT {
                r#type: INPUT_KEYBOARD,
                Anonymous: INPUT_0 {
                    ki: KEYBDINPUT {
                        wVk: VIRTUAL_KEY(vk),
                        wScan: scan_code,
                        dwFlags: KEYEVENTF_SCANCODE,
                        time: 0,
                        dwExtraInfo: 0,
                    },
                },
            };
            unsafe {
                SendInput(&[input], std::mem::size_of::<INPUT>() as i32);
            }
        }

        fn send_key_up(&self, vk: u16, scan_code: u16) {
            let input = INPUT {
                r#type: INPUT_KEYBOARD,
                Anonymous: INPUT_0 {
                    ki: KEYBDINPUT {
                        wVk: VIRTUAL_KEY(vk),
                        wScan: scan_code,
                        dwFlags: KEYEVENTF_SCANCODE | KEYEVENTF_KEYUP,
                        time: 0,
                        dwExtraInfo: 0,
                    },
                },
            };
            unsafe {
                SendInput(&[input], std::mem::size_of::<INPUT>() as i32);
            }
        }

        fn send_game_key_down(&self, scan_code: u16) {
            // 使用 vk=0xFF，游戏能识别，聊天框不识别
            let input = INPUT {
                r#type: INPUT_KEYBOARD,
                Anonymous: INPUT_0 {
                    ki: KEYBDINPUT {
                        wVk: VIRTUAL_KEY(0xFF),
                        wScan: scan_code,
                        dwFlags: KEYBD_EVENT_FLAGS(0),
                        time: 0,
                        dwExtraInfo: 0,
                    },
                },
            };
            unsafe {
                SendInput(&[input], std::mem::size_of::<INPUT>() as i32);
            }
        }

        fn send_game_key_up(&self, scan_code: u16) {
            let input = INPUT {
                r#type: INPUT_KEYBOARD,
                Anonymous: INPUT_0 {
                    ki: KEYBDINPUT {
                        wVk: VIRTUAL_KEY(0xFF),
                        wScan: scan_code,
                        dwFlags: KEYEVENTF_KEYUP,
                        time: 0,
                        dwExtraInfo: 0,
                    },
                },
            };
            unsafe {
                SendInput(&[input], std::mem::size_of::<INPUT>() as i32);
            }
        }

        fn is_key_pressed(&self, vk: u16) -> bool {
            unsafe { GetAsyncKeyState(vk as i32) < 0 }
        }
    }
}

#[cfg(windows)]
pub use windows_impl::WindowsKeyboardDriver;
