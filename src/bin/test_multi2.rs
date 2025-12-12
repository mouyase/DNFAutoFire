//! 多键连发测试 v2
//!
//! 基于 test_final.rs 修改
//! 多键逻辑：按住 A+B 时，每个周期依次发送 A, B
//!
//! 测试按键：L, K, J
//! 按 ESC 退出

use std::sync::atomic::{AtomicBool, AtomicU32, Ordering};
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
    KEYEVENTF_KEYUP, KEYEVENTF_SCANCODE, KEYBD_EVENT_FLAGS, VIRTUAL_KEY,
    MapVirtualKeyW, MAPVK_VK_TO_VSC,
};
use windows::Win32::System::LibraryLoader::GetModuleHandleW;
use windows::Win32::System::Threading::{
    GetCurrentThread, SetThreadPriority, THREAD_PRIORITY_ABOVE_NORMAL,
};

// 连发间隔
const KEY_DOWN_MS: u64 = 16;
const KEY_UP_MS: u64 = 17;

const VK_J: u32 = 0x4A;
const VK_K: u32 = 0x4B;
const VK_L: u32 = 0x4C;
const VK_ESCAPE: u32 = 0x1B;
const WM_KEYDOWN: u32 = 0x0100;
const WM_KEYUP: u32 = 0x0101;

// 用位掩码表示按键状态
static PRESSED_KEYS: AtomicU32 = AtomicU32::new(0);
static SHOULD_EXIT: AtomicBool = AtomicBool::new(false);

const KEY_BIT_J: u32 = 1 << 0;
const KEY_BIT_K: u32 = 1 << 1;
const KEY_BIT_L: u32 = 1 << 2;

fn vk_to_bit(vk: u32) -> u32 {
    match vk {
        VK_J => KEY_BIT_J,
        VK_K => KEY_BIT_K,
        VK_L => KEY_BIT_L,
        _ => 0,
    }
}

fn vk_to_sc(vk: u32) -> u16 {
    unsafe { MapVirtualKeyW(vk, MAPVK_VK_TO_VSC) as u16 }
}

fn get_key_name(vk: u32) -> &'static str {
    match vk {
        VK_J => "J",
        VK_K => "K",
        VK_L => "L",
        _ => "?",
    }
}

fn main() {
    println!("=== 多键连发测试 v2 ===");
    println!("请以管理员身份运行！\n");
    println!("连发按键：J, K, L");
    println!("多键逻辑：按住 A+B 时，依次发送 A, B, A, B...");
    println!("按 ESC：退出程序");
    println!("---\n");

    println!("J: vk={:#04X}, sc={:#04X}", VK_J, vk_to_sc(VK_J));
    println!("K: vk={:#04X}, sc={:#04X}", VK_K, vk_to_sc(VK_K));
    println!("L: vk={:#04X}, sc={:#04X}", VK_L, vk_to_sc(VK_L));

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

        println!("\n键盘钩子已安装\n");

        let mut msg = MSG::default();
        while !SHOULD_EXIT.load(Ordering::SeqCst) {
            if GetMessageW(&mut msg, None, 0, 0).as_bool() {
                // 处理消息
            }
        }

        let _ = UnhookWindowsHookEx(hook);
        println!("\n键盘钩子已卸载");
    }

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
            return CallNextHookEx(None, code, wparam, lparam);
        }

        // 检查是否是我们要处理的按键
        let key_bit = vk_to_bit(vk);
        if key_bit != 0 && !is_injected {
            if is_keydown {
                let old = PRESSED_KEYS.fetch_or(key_bit, Ordering::SeqCst);
                if old & key_bit == 0 {
                    // 首次按下
                    println!("[Hook] {} 按下", get_key_name(vk));
                    // 发送一次正常按键（聊天框能识别）
                    send_key_normal(vk as u16, vk_to_sc(vk), false);
                }
                return LRESULT(1);
            } else if is_keyup {
                PRESSED_KEYS.fetch_and(!key_bit, Ordering::SeqCst);
                println!("[Hook] {} 释放", get_key_name(vk));
                // 发送正常释放
                send_key_normal(vk as u16, vk_to_sc(vk), true);
                return LRESULT(1);
            }
        }
    }

    CallNextHookEx(None, code, wparam, lparam)
}

fn autofire_loop() {
    println!("[AutoFire] 连发线程已启动");

    unsafe {
        let thread = GetCurrentThread();
        let _ = SetThreadPriority(thread, THREAD_PRIORITY_ABOVE_NORMAL);
    }

    // 按键列表（按顺序）
    let keys: [(u32, u16); 3] = [
        (VK_J, vk_to_sc(VK_J)),
        (VK_K, vk_to_sc(VK_K)),
        (VK_L, vk_to_sc(VK_L)),
    ];

    let mut send_count: u64 = 0;

    loop {
        if SHOULD_EXIT.load(Ordering::SeqCst) {
            break;
        }

        let pressed = PRESSED_KEYS.load(Ordering::SeqCst);

        if pressed != 0 {
            // 依次发送每个按住的键
            for &(vk, sc) in &keys {
                let bit = vk_to_bit(vk);
                if pressed & bit != 0 {
                    // 发送这个键
                    send_key_game_only(sc, false);
                    thread::sleep(Duration::from_millis(KEY_DOWN_MS));
                    send_key_game_only(sc, true);
                    thread::sleep(Duration::from_millis(KEY_UP_MS));

                    send_count += 1;
                }
            }

            if send_count % 30 == 0 && send_count > 0 {
                println!("[AutoFire] 已发送 {} 次", send_count);
            }
        } else {
            send_count = 0;
            thread::sleep(Duration::from_millis(5));
        }
    }

    println!("[AutoFire] 连发线程已停止");
}

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
                wVk: VIRTUAL_KEY(vk),
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

fn send_key_game_only(sc: u16, key_up: bool) {
    let flags = if key_up {
        KEYEVENTF_KEYUP
    } else {
        KEYBD_EVENT_FLAGS(0)
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
