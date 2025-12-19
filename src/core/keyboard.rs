//! 键盘输入模块
//!
//! 提供 Windows SendInput API 封装，用于发送按键到游戏

use std::fmt::Debug;

/// 常用按键的虚拟键码
#[allow(dead_code)]
pub mod vk {
    // 字母键 A-Z
    pub const VK_A: u16 = 0x41;
    pub const VK_D: u16 = 0x44;
    pub const VK_E: u16 = 0x45;
    pub const VK_F: u16 = 0x46;
    pub const VK_G: u16 = 0x47;
    pub const VK_H: u16 = 0x48;
    pub const VK_J: u16 = 0x4A;
    pub const VK_K: u16 = 0x4B;
    pub const VK_L: u16 = 0x4C;
    pub const VK_Q: u16 = 0x51;
    pub const VK_R: u16 = 0x52;
    pub const VK_S: u16 = 0x53;
    pub const VK_T: u16 = 0x54;
    pub const VK_W: u16 = 0x57;
    pub const VK_X: u16 = 0x58;
    pub const VK_Y: u16 = 0x59;
    pub const VK_Z: u16 = 0x5A;

    // 数字键 1-9 (DNF 常用技能快捷键)
    pub const VK_1: u16 = 0x31;
    pub const VK_2: u16 = 0x32;
    pub const VK_3: u16 = 0x33;
    pub const VK_4: u16 = 0x34;
    pub const VK_5: u16 = 0x35;
    pub const VK_6: u16 = 0x36;
    pub const VK_7: u16 = 0x37;
    pub const VK_8: u16 = 0x38;
    pub const VK_9: u16 = 0x39;

    // 功能键和控制键
    pub const VK_SPACE: u16 = 0x20;
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
