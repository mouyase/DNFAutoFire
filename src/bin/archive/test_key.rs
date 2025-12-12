//! 按键测试程序 - 测试输入法兼容
//!
//! 数字键 1: vk=0 方式（之前的）
//! 数字键 2: vk=0xFF 方式（AHK的做法，输入法不识别）

use std::thread;
use std::time::{Duration, Instant};

use windows::Win32::UI::Input::KeyboardAndMouse::{
    SendInput, GetAsyncKeyState, INPUT, INPUT_0, INPUT_KEYBOARD, KEYBDINPUT,
    KEYEVENTF_KEYUP, KEYEVENTF_SCANCODE, VIRTUAL_KEY, MapVirtualKeyW, MAPVK_VK_TO_VSC,
};

const VK_1: u16 = 0x31;
const VK_2: u16 = 0x32;
const VK_L: u16 = 0x4C;

fn main() {
    println!("=== 输入法兼容测试 ===");
    println!("请以管理员身份运行！\n");
    println!("数字键 1: vk=0 方式（输入法可能会识别）");
    println!("数字键 2: vk=0xFF 方式（AHK做法，输入法不识别）");
    println!("\n先在记事本测试，看哪种方式输入法不会出字");
    println!("按 Ctrl+C 退出\n");

    let l_scan_code = unsafe { MapVirtualKeyW(VK_L as u32, MAPVK_VK_TO_VSC) as u16 };
    println!("L 键 ScanCode: {:#04X}", l_scan_code);
    println!("---");

    let mut last_print = Instant::now();
    let mut send_count: u64 = 0;

    loop {
        let key1 = is_key_pressed(VK_1);
        let key2 = is_key_pressed(VK_2);

        if key1 {
            send_count += 1;
            // 方式1: vk=0
            send_key_vk0(l_scan_code, false);
            thread::sleep(Duration::from_millis(16));
            send_key_vk0(l_scan_code, true);
            thread::sleep(Duration::from_millis(17));
        }
        else if key2 {
            send_count += 1;
            // 方式2: vk=0xFF（AHK的做法）
            send_key_vkff(l_scan_code, false);
            thread::sleep(Duration::from_millis(16));
            send_key_vkff(l_scan_code, true);
            thread::sleep(Duration::from_millis(17));
        }
        else {
            thread::sleep(Duration::from_millis(5));
        }

        if last_print.elapsed() > Duration::from_secs(1) {
            if send_count > 0 {
                println!("发送次数: {}/秒", send_count);
            }
            send_count = 0;
            last_print = Instant::now();
        }
    }
}

fn is_key_pressed(vk: u16) -> bool {
    unsafe { GetAsyncKeyState(vk as i32) < 0 }
}

/// 方式1: vk=0, 只用扫描码
fn send_key_vk0(scan_code: u16, key_up: bool) {
    let flags = if key_up {
        KEYEVENTF_SCANCODE | KEYEVENTF_KEYUP
    } else {
        KEYEVENTF_SCANCODE
    };

    let input = INPUT {
        r#type: INPUT_KEYBOARD,
        Anonymous: INPUT_0 {
            ki: KEYBDINPUT {
                wVk: VIRTUAL_KEY(0),  // vk = 0
                wScan: scan_code,
                dwFlags: flags,
                time: 0,
                dwExtraInfo: 0,
            },
        },
    };
    unsafe { SendInput(&[input], std::mem::size_of::<INPUT>() as i32); }
}

/// 方式2: vk=0xFF, AHK的做法，输入法不识别
fn send_key_vkff(scan_code: u16, key_up: bool) {
    let flags = if key_up {
        KEYEVENTF_SCANCODE | KEYEVENTF_KEYUP
    } else {
        KEYEVENTF_SCANCODE
    };

    let input = INPUT {
        r#type: INPUT_KEYBOARD,
        Anonymous: INPUT_0 {
            ki: KEYBDINPUT {
                wVk: VIRTUAL_KEY(0xFF),  // vk = 0xFF，无效虚拟键码
                wScan: scan_code,
                dwFlags: flags,
                time: 0,
                dwExtraInfo: 0,
            },
        },
    };
    unsafe { SendInput(&[input], std::mem::size_of::<INPUT>() as i32); }
}
