//! 测试 vkFF 方式 - 不用钩子，模拟 AHK 的做法
//!
//! AHK 的做法：
//! - 用 GetKeyState 检测物理按键
//! - 用 vkFFsc{XX} 发送（vk=0xFF，sc=真实扫描码）
//! - 不拦截原始按键
//!
//! 测试：
//! 按住 L 键，看游戏是否收到连发，聊天框是否只有一个 l
//! 按 Ctrl+C 退出

use std::thread;
use std::time::Duration;

use windows::Win32::UI::Input::KeyboardAndMouse::{
    SendInput, GetAsyncKeyState, INPUT, INPUT_0, INPUT_KEYBOARD, KEYBDINPUT,
    KEYEVENTF_KEYUP, KEYEVENTF_SCANCODE, VIRTUAL_KEY,
    MapVirtualKeyW, MAPVK_VK_TO_VSC,
};

const VK_L: u16 = 0x4C;

fn main() {
    println!("=== vkFF 测试（无钩子）===");
    println!("模拟 AHK 的 vkFFsc{{XX}} 方式\n");
    println!("按住 L 键：发送 vkFF + L扫描码");
    println!("测试：游戏是否连发？聊天框是否只有原始的 l？");
    println!("按 Ctrl+C 退出");
    println!("---\n");

    let l_scan = unsafe { MapVirtualKeyW(VK_L as u32, MAPVK_VK_TO_VSC) as u16 };
    println!("L 键 ScanCode: {:#04X}", l_scan);

    let mut send_count: u64 = 0;
    let mut was_pressed = false;

    loop {
        let is_pressed = is_key_pressed(VK_L);

        if is_pressed {
            if !was_pressed {
                println!("[检测] L 键按下 - 开始发送");
            }
            was_pressed = true;

            // 用 vkFF 方式发送
            send_key_vkff(l_scan, false); // down
            thread::sleep(Duration::from_millis(16));
            send_key_vkff(l_scan, true);  // up
            thread::sleep(Duration::from_millis(17));

            send_count += 1;
            if send_count % 30 == 0 {
                println!("[发送] 已发送 {} 次", send_count);
            }
        } else {
            if was_pressed {
                println!("[检测] L 键释放 - 停止发送，共 {} 次", send_count);
                send_count = 0;
            }
            was_pressed = false;
            thread::sleep(Duration::from_millis(5));
        }
    }
}

fn is_key_pressed(vk: u16) -> bool {
    unsafe { GetAsyncKeyState(vk as i32) < 0 }
}

/// vkFF 方式发送（AHK 的做法）
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
                wVk: VIRTUAL_KEY(0xFF), // vk=0xFF，输入法不识别
                wScan: scan_code,       // 真实扫描码，游戏能识别
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
