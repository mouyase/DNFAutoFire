//! 键盘输入模块 - 直接调用 Win32 SendInput API
//!
//! 这个模块提供高性能的键盘输入模拟
//!
//! 关键技术发现：
//! - 使用 vkFF + 真实扫描码 + 无 KEYEVENTF_SCANCODE 标志
//! - 游戏能识别按键，聊天框不会收到输入
//! - 这是经过多次测试验证的最佳方案

use windows::Win32::UI::Input::KeyboardAndMouse::{
    SendInput, GetAsyncKeyState, INPUT, INPUT_0, INPUT_KEYBOARD, KEYBDINPUT,
    KEYEVENTF_KEYUP, KEYEVENTF_SCANCODE, KEYBD_EVENT_FLAGS,
    VIRTUAL_KEY, MapVirtualKeyW, MAPVK_VK_TO_VSC,
};

/// 虚拟键码到扫描码的映射
/// DNF 等游戏通常需要扫描码而非虚拟键码
pub fn vk_to_scan_code(vk: u16) -> u16 {
    unsafe { MapVirtualKeyW(vk as u32, MAPVK_VK_TO_VSC) as u16 }
}

/// 发送按键按下事件
/// 使用扫描码以确保游戏能正确识别
#[inline]
pub fn send_key_down(scan_code: u16) {
    let input = INPUT {
        r#type: INPUT_KEYBOARD,
        Anonymous: INPUT_0 {
            ki: KEYBDINPUT {
                wVk: VIRTUAL_KEY(0), // 不使用虚拟键码
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

/// 发送按键释放事件
#[inline]
pub fn send_key_up(scan_code: u16) {
    let input = INPUT {
        r#type: INPUT_KEYBOARD,
        Anonymous: INPUT_0 {
            ki: KEYBDINPUT {
                wVk: VIRTUAL_KEY(0),
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

/// 发送完整的按键事件（按下+释放）
/// 这是连发的核心函数，优化为一次 SendInput 调用
#[inline]
pub fn send_key_press(scan_code: u16) {
    let inputs = [
        // 按下
        INPUT {
            r#type: INPUT_KEYBOARD,
            Anonymous: INPUT_0 {
                ki: KEYBDINPUT {
                    wVk: VIRTUAL_KEY(0),
                    wScan: scan_code,
                    dwFlags: KEYEVENTF_SCANCODE,
                    time: 0,
                    dwExtraInfo: 0,
                },
            },
        },
        // 释放
        INPUT {
            r#type: INPUT_KEYBOARD,
            Anonymous: INPUT_0 {
                ki: KEYBDINPUT {
                    wVk: VIRTUAL_KEY(0),
                    wScan: scan_code,
                    dwFlags: KEYEVENTF_SCANCODE | KEYEVENTF_KEYUP,
                    time: 0,
                    dwExtraInfo: 0,
                },
            },
        },
    ];

    unsafe {
        // 一次调用发送两个事件，比分开调用更快
        SendInput(&inputs, std::mem::size_of::<INPUT>() as i32);
    }
}

/// 检测按键是否被物理按下
/// 使用 GetAsyncKeyState 而非 GetKeyState，获取全局状态
#[inline]
pub fn is_key_pressed(vk: u16) -> bool {
    unsafe {
        // 最高位为1表示按键当前被按下
        GetAsyncKeyState(vk as i32) < 0
    }
}

/// 发送"游戏专用"按键 - 游戏能识别，聊天框不会收到
///
/// 技术原理：
/// - 使用 vkFF（无效虚拟键码）+ 真实扫描码
/// - 不设置 KEYEVENTF_SCANCODE 标志
/// - 游戏通过扫描码识别按键，但输入法/聊天框因 vk 无效而忽略
#[inline]
pub fn send_key_game_only_down(scan_code: u16) {
    let input = INPUT {
        r#type: INPUT_KEYBOARD,
        Anonymous: INPUT_0 {
            ki: KEYBDINPUT {
                wVk: VIRTUAL_KEY(0xFF),  // 无效 VK
                wScan: scan_code,
                dwFlags: KEYBD_EVENT_FLAGS(0),  // 无标志
                time: 0,
                dwExtraInfo: 0,
            },
        },
    };

    unsafe {
        SendInput(&[input], std::mem::size_of::<INPUT>() as i32);
    }
}

/// 发送"游戏专用"按键释放
#[inline]
pub fn send_key_game_only_up(scan_code: u16) {
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

/// 发送正常按键（聊天框可识别）- 用于首次按下
#[inline]
pub fn send_key_normal_down(vk: u16, scan_code: u16) {
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

/// 发送正常按键释放
#[inline]
pub fn send_key_normal_up(vk: u16, scan_code: u16) {
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

/// 常用按键的虚拟键码
pub mod vk {
    pub const VK_BACK: u16 = 0x08;
    pub const VK_TAB: u16 = 0x09;
    pub const VK_RETURN: u16 = 0x0D;
    pub const VK_SHIFT: u16 = 0x10;
    pub const VK_CONTROL: u16 = 0x11;
    pub const VK_MENU: u16 = 0x12; // Alt
    pub const VK_PAUSE: u16 = 0x13;
    pub const VK_CAPITAL: u16 = 0x14; // Caps Lock
    pub const VK_ESCAPE: u16 = 0x1B;
    pub const VK_SPACE: u16 = 0x20;
    pub const VK_PRIOR: u16 = 0x21; // Page Up
    pub const VK_NEXT: u16 = 0x22;  // Page Down
    pub const VK_END: u16 = 0x23;
    pub const VK_HOME: u16 = 0x24;
    pub const VK_LEFT: u16 = 0x25;
    pub const VK_UP: u16 = 0x26;
    pub const VK_RIGHT: u16 = 0x27;
    pub const VK_DOWN: u16 = 0x28;
    pub const VK_SNAPSHOT: u16 = 0x2C; // Print Screen
    pub const VK_INSERT: u16 = 0x2D;
    pub const VK_DELETE: u16 = 0x2E;

    // 字母键 A-Z (0x41-0x5A)
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

    // 数字键 0-9 (0x30-0x39)
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

    // 小键盘数字
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

    // 小键盘运算符
    pub const VK_MULTIPLY: u16 = 0x6A;
    pub const VK_ADD: u16 = 0x6B;
    pub const VK_SEPARATOR: u16 = 0x6C;
    pub const VK_SUBTRACT: u16 = 0x6D;
    pub const VK_DECIMAL: u16 = 0x6E;
    pub const VK_DIVIDE: u16 = 0x6F;

    // 锁定键
    pub const VK_NUMLOCK: u16 = 0x90;
    pub const VK_SCROLL: u16 = 0x91;

    // 左右修饰键
    pub const VK_LSHIFT: u16 = 0xA0;
    pub const VK_RSHIFT: u16 = 0xA1;
    pub const VK_LCONTROL: u16 = 0xA2;
    pub const VK_RCONTROL: u16 = 0xA3;
    pub const VK_LMENU: u16 = 0xA4;
    pub const VK_RMENU: u16 = 0xA5;

    // Windows 键和应用键
    pub const VK_LWIN: u16 = 0x5B;
    pub const VK_RWIN: u16 = 0x5C;
    pub const VK_APPS: u16 = 0x5D;

    // OEM 键（符号键）
    pub const VK_OEM_1: u16 = 0xBA;      // ;:
    pub const VK_OEM_PLUS: u16 = 0xBB;   // =+
    pub const VK_OEM_COMMA: u16 = 0xBC;  // ,<
    pub const VK_OEM_MINUS: u16 = 0xBD;  // -_
    pub const VK_OEM_PERIOD: u16 = 0xBE; // .>
    pub const VK_OEM_2: u16 = 0xBF;      // /?
    pub const VK_OEM_3: u16 = 0xC0;      // `~
    pub const VK_OEM_4: u16 = 0xDB;      // [{
    pub const VK_OEM_5: u16 = 0xDC;      // \|
    pub const VK_OEM_6: u16 = 0xDD;      // ]}
    pub const VK_OEM_7: u16 = 0xDE;      // '"

    /// 根据按键名获取虚拟键码
    pub fn from_name(name: &str) -> Option<u16> {
        match name.to_uppercase().as_str() {
            "A" => Some(VK_A),
            "B" => Some(VK_B),
            "C" => Some(VK_C),
            "D" => Some(VK_D),
            "E" => Some(VK_E),
            "F" => Some(VK_F),
            "G" => Some(VK_G),
            "H" => Some(VK_H),
            "I" => Some(VK_I),
            "J" => Some(VK_J),
            "K" => Some(VK_K),
            "L" => Some(VK_L),
            "M" => Some(VK_M),
            "N" => Some(VK_N),
            "O" => Some(VK_O),
            "P" => Some(VK_P),
            "Q" => Some(VK_Q),
            "R" => Some(VK_R),
            "S" => Some(VK_S),
            "T" => Some(VK_T),
            "U" => Some(VK_U),
            "V" => Some(VK_V),
            "W" => Some(VK_W),
            "X" => Some(VK_X),
            "Y" => Some(VK_Y),
            "Z" => Some(VK_Z),
            "0" => Some(VK_0),
            "1" => Some(VK_1),
            "2" => Some(VK_2),
            "3" => Some(VK_3),
            "4" => Some(VK_4),
            "5" => Some(VK_5),
            "6" => Some(VK_6),
            "7" => Some(VK_7),
            "8" => Some(VK_8),
            "9" => Some(VK_9),
            "F1" => Some(VK_F1),
            "F2" => Some(VK_F2),
            "F3" => Some(VK_F3),
            "F4" => Some(VK_F4),
            "F5" => Some(VK_F5),
            "F6" => Some(VK_F6),
            "F7" => Some(VK_F7),
            "F8" => Some(VK_F8),
            "F9" => Some(VK_F9),
            "F10" => Some(VK_F10),
            "F11" => Some(VK_F11),
            "F12" => Some(VK_F12),
            "SPACE" | " " => Some(VK_SPACE),
            "TAB" => Some(VK_TAB),
            "ENTER" | "RETURN" => Some(VK_RETURN),
            "ESC" | "ESCAPE" => Some(VK_ESCAPE),
            "BACKSPACE" | "BACK" => Some(VK_BACK),
            "SHIFT" => Some(VK_SHIFT),
            "CTRL" | "CONTROL" => Some(VK_CONTROL),
            "ALT" | "MENU" => Some(VK_MENU),
            "LEFT" => Some(VK_LEFT),
            "UP" => Some(VK_UP),
            "RIGHT" => Some(VK_RIGHT),
            "DOWN" => Some(VK_DOWN),
            "NUMPAD0" | "NUM0" => Some(VK_NUMPAD0),
            "NUMPAD1" | "NUM1" => Some(VK_NUMPAD1),
            "NUMPAD2" | "NUM2" => Some(VK_NUMPAD2),
            "NUMPAD3" | "NUM3" => Some(VK_NUMPAD3),
            "NUMPAD4" | "NUM4" => Some(VK_NUMPAD4),
            "NUMPAD5" | "NUM5" => Some(VK_NUMPAD5),
            "NUMPAD6" | "NUM6" => Some(VK_NUMPAD6),
            "NUMPAD7" | "NUM7" => Some(VK_NUMPAD7),
            "NUMPAD8" | "NUM8" => Some(VK_NUMPAD8),
            "NUMPAD9" | "NUM9" => Some(VK_NUMPAD9),
            _ => None,
        }
    }

    /// 获取按键的显示名称
    pub fn to_name(vk: u16) -> &'static str {
        match vk {
            VK_A => "A", VK_B => "B", VK_C => "C", VK_D => "D",
            VK_E => "E", VK_F => "F", VK_G => "G", VK_H => "H",
            VK_I => "I", VK_J => "J", VK_K => "K", VK_L => "L",
            VK_M => "M", VK_N => "N", VK_O => "O", VK_P => "P",
            VK_Q => "Q", VK_R => "R", VK_S => "S", VK_T => "T",
            VK_U => "U", VK_V => "V", VK_W => "W", VK_X => "X",
            VK_Y => "Y", VK_Z => "Z",
            VK_0 => "0", VK_1 => "1", VK_2 => "2", VK_3 => "3",
            VK_4 => "4", VK_5 => "5", VK_6 => "6", VK_7 => "7",
            VK_8 => "8", VK_9 => "9",
            VK_F1 => "F1", VK_F2 => "F2", VK_F3 => "F3", VK_F4 => "F4",
            VK_F5 => "F5", VK_F6 => "F6", VK_F7 => "F7", VK_F8 => "F8",
            VK_F9 => "F9", VK_F10 => "F10", VK_F11 => "F11", VK_F12 => "F12",
            VK_SPACE => "Space",
            VK_TAB => "Tab",
            VK_RETURN => "Enter",
            VK_ESCAPE => "Esc",
            VK_BACK => "Back",
            VK_SHIFT => "Shift",
            VK_CONTROL => "Ctrl",
            VK_MENU => "Alt",
            VK_LEFT => "Left",
            VK_UP => "Up",
            VK_RIGHT => "Right",
            VK_DOWN => "Down",
            VK_NUMPAD0 => "Num0", VK_NUMPAD1 => "Num1", VK_NUMPAD2 => "Num2",
            VK_NUMPAD3 => "Num3", VK_NUMPAD4 => "Num4", VK_NUMPAD5 => "Num5",
            VK_NUMPAD6 => "Num6", VK_NUMPAD7 => "Num7", VK_NUMPAD8 => "Num8",
            VK_NUMPAD9 => "Num9",
            _ => "?",
        }
    }
}
