//! 测试不同的 dwFlags 组合
//!
//! 数字键 1：有 KEYEVENTF_SCANCODE 标志（之前的方式）
//! 数字键 2：无 KEYEVENTF_SCANCODE 标志（wScan 可能被忽略）
//! 数字键 3：只用 wVk，不设置 wScan
//!
//! 按 Ctrl+C 退出

use std::thread;
use std::time::Duration;

use windows::Win32::UI::Input::KeyboardAndMouse::{
    SendInput, GetAsyncKeyState, INPUT, INPUT_0, INPUT_KEYBOARD, KEYBDINPUT,
    KEYEVENTF_KEYUP, KEYEVENTF_SCANCODE, KEYBD_EVENT_FLAGS, VIRTUAL_KEY,
    MapVirtualKeyW, MAPVK_VK_TO_VSC,
};

const VK_1: u16 = 0x31;
const VK_2: u16 = 0x32;
const VK_3: u16 = 0x33;
const VK_L: u16 = 0x4C;

fn main() {
    println!("=== dwFlags 组合测试 ===\n");
    println!("数字键 1：vkFF + sc + KEYEVENTF_SCANCODE（之前的方式）");
    println!("数字键 2：vkFF + sc + 无 SCANCODE 标志");
    println!("数字键 3：只用真实 vk，无 SCANCODE 标志");
    println!("\n分别在游戏战斗和聊天框测试！");
    println!("按 Ctrl+C 退出");
    println!("---\n");

    let l_vk = VK_L;
    let l_sc = unsafe { MapVirtualKeyW(VK_L as u32, MAPVK_VK_TO_VSC) as u16 };
    println!("L 键: vk={:#04X}, sc={:#04X}\n", l_vk, l_sc);

    loop {
        // 方式1：vkFF + sc + KEYEVENTF_SCANCODE
        if is_key_pressed(VK_1) {
            println!("[1] vkFF + sc + SCANCODE flag");
            send_key_method1(l_sc, false);
            thread::sleep(Duration::from_millis(16));
            send_key_method1(l_sc, true);
            thread::sleep(Duration::from_millis(50));
        }
        // 方式2：vkFF + sc + 无 SCANCODE 标志
        else if is_key_pressed(VK_2) {
            println!("[2] vkFF + sc + NO SCANCODE flag");
            send_key_method2(l_sc, false);
            thread::sleep(Duration::from_millis(16));
            send_key_method2(l_sc, true);
            thread::sleep(Duration::from_millis(50));
        }
        // 方式3：只用真实 vk，无 SCANCODE 标志
        else if is_key_pressed(VK_3) {
            println!("[3] real vk + NO SCANCODE flag");
            send_key_method3(l_vk, false);
            thread::sleep(Duration::from_millis(16));
            send_key_method3(l_vk, true);
            thread::sleep(Duration::from_millis(50));
        }
        else {
            thread::sleep(Duration::from_millis(10));
        }
    }
}

fn is_key_pressed(vk: u16) -> bool {
    unsafe { GetAsyncKeyState(vk as i32) < 0 }
}

/// 方式1：vkFF + sc + KEYEVENTF_SCANCODE（之前的方式）
fn send_key_method1(sc: u16, key_up: bool) {
    let flags = if key_up {
        KEYEVENTF_SCANCODE | KEYEVENTF_KEYUP
    } else {
        KEYEVENTF_SCANCODE
    };

    let input = INPUT {
        r#type: INPUT_KEYBOARD,
        Anonymous: INPUT_0 {
            ki: KEYBDINPUT {
                wVk: VIRTUAL_KEY(0xFF),
                wScan: sc,
                dwFlags: flags,
                time: 0,
                dwExtraInfo: 0,
            },
        },
    };

    unsafe {
        SendInput(&[input], std::mem::size_of::<INPUT>() as i32);
    }
}

/// 方式2：vkFF + sc + 无 SCANCODE 标志
fn send_key_method2(sc: u16, key_up: bool) {
    let flags = if key_up {
        KEYEVENTF_KEYUP
    } else {
        KEYBD_EVENT_FLAGS(0)  // 无标志
    };

    let input = INPUT {
        r#type: INPUT_KEYBOARD,
        Anonymous: INPUT_0 {
            ki: KEYBDINPUT {
                wVk: VIRTUAL_KEY(0xFF),
                wScan: sc,
                dwFlags: flags,
                time: 0,
                dwExtraInfo: 0,
            },
        },
    };

    unsafe {
        SendInput(&[input], std::mem::size_of::<INPUT>() as i32);
    }
}

/// 方式3：只用真实 vk，无 SCANCODE 标志
fn send_key_method3(vk: u16, key_up: bool) {
    let flags = if key_up {
        KEYEVENTF_KEYUP
    } else {
        KEYBD_EVENT_FLAGS(0)
    };

    let input = INPUT {
        r#type: INPUT_KEYBOARD,
        Anonymous: INPUT_0 {
            ki: KEYBDINPUT {
                wVk: VIRTUAL_KEY(vk),
                wScan: 0,
                dwFlags: flags,
                time: 0,
                dwExtraInfo: 0,
            },
        },
    };

    unsafe {
        SendInput(&[input], std::mem::size_of::<INPUT>() as i32);
    }
}
