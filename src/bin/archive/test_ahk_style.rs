//! 模拟 AHK 的完整机制
//!
//! AHK 的做法：
//! 1. 钩子拦截原始按键
//! 2. 立即发送一次正常按键（有正常 vk+sc，聊天框能识别）
//! 3. 连发线程用 vkFF+sc 发送（聊天框可能不识别，游戏识别）
//! 4. 松开时发送一次正常的 Up
//!
//! 按 ESC 退出

use std::sync::atomic::{AtomicBool, AtomicU16, Ordering};
use std::thread;
use std::time::Duration;

use windows::Win32::Foundation::{LPARAM, LRESULT, WPARAM};
use windows::Win32::UI::WindowsAndMessaging::{
    SetWindowsHookExW, UnhookWindowsHookEx, CallNextHookEx,
    GetMessageW, MSG, WH_KEYBOARD_LL, KBDLLHOOKSTRUCT,
    LLKHF_INJECTED, LLKHF_LOWER_IL_INJECTED,
};
use windows::Win32::UI::Input::KeyboardAndMouse::{
    SendInput, INPUT, INPUT_0, INPUT_KEYBOARD, KEYBDINPUT,
    KEYEVENTF_KEYUP, KEYEVENTF_SCANCODE, VIRTUAL_KEY,
    MapVirtualKeyW, MAPVK_VK_TO_VSC,
};
use windows::Win32::System::LibraryLoader::GetModuleHandleW;

const VK_L: u16 = 0x4C;
const VK_ESCAPE: u32 = 0x1B;
const WM_KEYDOWN: u32 = 0x0100;
const WM_KEYUP: u32 = 0x0101;

// 全局状态
static L_KEY_DOWN: AtomicBool = AtomicBool::new(false);
static SHOULD_EXIT: AtomicBool = AtomicBool::new(false);
static L_VK: AtomicU16 = AtomicU16::new(0);
static L_SC: AtomicU16 = AtomicU16::new(0);

fn main() {
    println!("=== 模拟 AHK 机制 ===");
    println!("请以管理员身份运行！\n");
    println!("按住 L 键：");
    println!("  1. 拦截原始按键");
    println!("  2. 发送一次正常按键（vk+sc，聊天框能识别）");
    println!("  3. 连发用 vkFF+sc（聊天框不识别，游戏识别）");
    println!("按 ESC：退出程序");
    println!("---\n");

    L_VK.store(VK_L, Ordering::SeqCst);
    let l_scan = unsafe { MapVirtualKeyW(VK_L as u32, MAPVK_VK_TO_VSC) as u16 };
    L_SC.store(l_scan, Ordering::SeqCst);
    println!("L 键: vk={:#04X}, sc={:#04X}", VK_L, l_scan);

    // 启动连发线程
    let autofire_thread = thread::spawn(|| {
        autofire_loop();
    });

    // 安装键盘钩子
    unsafe {
        let hook = SetWindowsHookExW(
            WH_KEYBOARD_LL,
            Some(keyboard_hook_proc),
            GetModuleHandleW(None).unwrap_or_default(),
            0,
        ).expect("Failed to set keyboard hook");

        println!("键盘钩子已安装\n");

        let mut msg = MSG::default();
        while !SHOULD_EXIT.load(Ordering::SeqCst) {
            if GetMessageW(&mut msg, None, 0, 0).as_bool() {
                // 处理消息
            }
        }

        let _ = UnhookWindowsHookEx(hook);
        println!("\n键盘钩子已卸载");
    }

    L_KEY_DOWN.store(false, Ordering::SeqCst);
    SHOULD_EXIT.store(true, Ordering::SeqCst);
    let _ = autofire_thread.join();

    println!("程序已退出");
}

unsafe extern "system" fn keyboard_hook_proc(
    code: i32,
    wparam: WPARAM,
    lparam: LPARAM,
) -> LRESULT {
    if code >= 0 {
        let kb = *(lparam.0 as *const KBDLLHOOKSTRUCT);
        let vk = kb.vkCode;
        let is_keydown = wparam.0 as u32 == WM_KEYDOWN;
        let is_keyup = wparam.0 as u32 == WM_KEYUP;

        let is_injected = (kb.flags.0 & LLKHF_INJECTED.0) != 0
            || (kb.flags.0 & LLKHF_LOWER_IL_INJECTED.0) != 0;

        // ESC 退出
        if vk == VK_ESCAPE && is_keydown {
            SHOULD_EXIT.store(true, Ordering::SeqCst);
            windows::Win32::UI::WindowsAndMessaging::PostQuitMessage(0);
        }

        // L 键处理
        if vk == VK_L as u32 && !is_injected {
            if is_keydown {
                if !L_KEY_DOWN.load(Ordering::SeqCst) {
                    // 首次按下
                    println!("[Hook] L 首次按下 - 发送正常按键 + 开始连发");
                    L_KEY_DOWN.store(true, Ordering::SeqCst);

                    // 发送一次正常按键（有正常 vk+sc，聊天框能识别）
                    let vk = L_VK.load(Ordering::SeqCst);
                    let sc = L_SC.load(Ordering::SeqCst);
                    send_key_normal(vk, sc, false);
                }
                // 拦截原始按键（包括系统重复）
                return LRESULT(1);
            } else if is_keyup {
                println!("[Hook] L 释放 - 发送正常释放 + 停止连发");
                L_KEY_DOWN.store(false, Ordering::SeqCst);

                // 发送正常释放
                let vk = L_VK.load(Ordering::SeqCst);
                let sc = L_SC.load(Ordering::SeqCst);
                send_key_normal(vk, sc, true);

                // 拦截原始释放
                return LRESULT(1);
            }
        }
    }

    CallNextHookEx(None, code, wparam, lparam)
}

fn autofire_loop() {
    println!("[AutoFire] 连发线程已启动");

    let mut send_count: u64 = 0;
    let mut first_press = true;

    while !SHOULD_EXIT.load(Ordering::SeqCst) {
        if L_KEY_DOWN.load(Ordering::SeqCst) {
            // 首次按下时，等待正常按键先发送
            if first_press {
                thread::sleep(Duration::from_millis(5));
                first_press = false;
            }

            let sc = L_SC.load(Ordering::SeqCst);

            // 用 vkFF + sc 发送（游戏能识别，聊天框可能不识别）
            send_key_vkff(sc, false);
            thread::sleep(Duration::from_millis(16));
            send_key_vkff(sc, true);
            thread::sleep(Duration::from_millis(17));

            send_count += 1;
            if send_count % 30 == 0 {
                println!("[AutoFire] 已发送 {} 次", send_count);
            }
        } else {
            first_press = true;
            send_count = 0;
            thread::sleep(Duration::from_millis(5));
        }
    }

    println!("[AutoFire] 连发线程已停止");
}

/// 发送正常按键（有正常 vk 和 sc）
fn send_key_normal(vk: u16, sc: u16, key_up: bool) {
    let flags = if key_up {
        KEYEVENTF_SCANCODE | KEYEVENTF_KEYUP
    } else {
        KEYEVENTF_SCANCODE
    };

    let input = INPUT {
        r#type: INPUT_KEYBOARD,
        Anonymous: INPUT_0 {
            ki: KEYBDINPUT {
                wVk: VIRTUAL_KEY(vk),  // 正常 vk
                wScan: sc,              // 正常 sc
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

/// 发送 vkFF + sc（游戏能识别，聊天框可能不识别）
fn send_key_vkff(sc: u16, key_up: bool) {
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
                wScan: sc,                // 真实 sc
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
