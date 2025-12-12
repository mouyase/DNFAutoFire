//! 测试 vk 和 sc 的不同组合
//!
//! 数字键 1：vkFF + 真实sc（游戏能感知，输入法不能）
//! 数字键 2：真实vk + scFF（输入法能感知，游戏不能）
//!
//! 在游戏战斗和聊天框里分别测试这两种方式
//! 按 Ctrl+C 退出

use std::thread;
use std::time::Duration;

use windows::Win32::UI::Input::KeyboardAndMouse::{
    SendInput, GetAsyncKeyState, INPUT, INPUT_0, INPUT_KEYBOARD, KEYBDINPUT,
    KEYEVENTF_KEYUP, KEYEVENTF_SCANCODE, VIRTUAL_KEY,
    MapVirtualKeyW, MAPVK_VK_TO_VSC,
};

const VK_1: u16 = 0x31;
const VK_2: u16 = 0x32;
const VK_L: u16 = 0x4C;

fn main() {
    println!("=== vk/sc 组合测试 ===\n");
    println!("数字键 1：vkFF + 真实sc（预期：游戏能感知，输入法不能）");
    println!("数字键 2：真实vk + scFF（预期：输入法能感知，游戏不能）");
    println!("\n请在游戏战斗和聊天框里分别测试！");
    println!("按 Ctrl+C 退出");
    println!("---\n");

    let l_vk = VK_L;
    let l_sc = unsafe { MapVirtualKeyW(VK_L as u32, MAPVK_VK_TO_VSC) as u16 };
    println!("L 键: vk={:#04X}, sc={:#04X}\n", l_vk, l_sc);

    loop {
        // 方式1：vkFF + 真实sc
        if is_key_pressed(VK_1) {
            println!("[方式1] vkFF + sc{:02X} - 发送中...", l_sc);
            send_key_vkff_sc_real(l_sc, false);
            thread::sleep(Duration::from_millis(16));
            send_key_vkff_sc_real(l_sc, true);
            thread::sleep(Duration::from_millis(50));
        }
        // 方式2：真实vk + scFF
        else if is_key_pressed(VK_2) {
            println!("[方式2] vk{:02X} + scFF - 发送中...", l_vk);
            send_key_vk_real_scff(l_vk, false);
            thread::sleep(Duration::from_millis(16));
            send_key_vk_real_scff(l_vk, true);
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

/// 方式1：vkFF + 真实扫描码
/// 预期：游戏能感知（读sc），输入法不能（vk无效）
fn send_key_vkff_sc_real(scan_code: u16, key_up: bool) {
    let flags = if key_up {
        KEYEVENTF_SCANCODE | KEYEVENTF_KEYUP
    } else {
        KEYEVENTF_SCANCODE
    };

    let input = INPUT {
        r#type: INPUT_KEYBOARD,
        Anonymous: INPUT_0 {
            ki: KEYBDINPUT {
                wVk: VIRTUAL_KEY(0xFF),  // 虚假 vk
                wScan: scan_code,         // 真实 sc
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

/// 方式2：真实虚拟码 + scFF
/// 预期：输入法能感知（读vk），游戏不能（sc无效）
fn send_key_vk_real_scff(vk: u16, key_up: bool) {
    let flags = if key_up {
        KEYEVENTF_KEYUP
    } else {
        KEYEVENTF_SCANCODE  // 不加这个标志，系统会自动从vk推算sc
    };

    // 注意：不加 KEYEVENTF_SCANCODE 时，wScan 会被忽略
    // 但我们想要 scFF，需要强制使用 SCANCODE 标志
    let flags_with_sc = if key_up {
        KEYEVENTF_SCANCODE | KEYEVENTF_KEYUP
    } else {
        KEYEVENTF_SCANCODE
    };

    let input = INPUT {
        r#type: INPUT_KEYBOARD,
        Anonymous: INPUT_0 {
            ki: KEYBDINPUT {
                wVk: VIRTUAL_KEY(vk),  // 真实 vk
                wScan: 0xFF,           // 虚假 sc
                dwFlags: flags_with_sc,
                time: 0,
                dwExtraInfo: 0,
            },
        },
    };

    unsafe {
        SendInput(&[input], std::mem::size_of::<INPUT>() as i32);
    }
}
