//! 多键连发测试
//!
//! 支持多个按键同时连发
//! 测试按键：L, K, J
//!
//! 按 ESC 退出

use std::collections::HashMap;
use std::sync::atomic::{AtomicBool, Ordering};
use std::thread;
use std::time::Duration;

use parking_lot::RwLock;
use once_cell::sync::Lazy;

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

const VK_ESCAPE: u32 = 0x1B;
const WM_KEYDOWN: u32 = 0x0100;
const WM_KEYUP: u32 = 0x0101;

// 连发间隔（毫秒）
const KEY_DOWN_MS: u64 = 16;
const KEY_UP_MS: u64 = 17;

// 全局状态
static SHOULD_EXIT: AtomicBool = AtomicBool::new(false);

// 按键配置：vk -> (sc, is_pressed)
static KEY_STATES: Lazy<RwLock<HashMap<u32, KeyState>>> = Lazy::new(|| {
    RwLock::new(HashMap::new())
});

#[derive(Clone)]
struct KeyState {
    vk: u16,
    sc: u16,
    pressed: bool,
}

fn main() {
    println!("=== 多键连发测试 ===");
    println!("请以管理员身份运行！\n");
    println!("连发按键：L, K, J");
    println!("间隔：{}ms + {}ms = {}ms", KEY_DOWN_MS, KEY_UP_MS, KEY_DOWN_MS + KEY_UP_MS);
    println!("按 ESC：退出程序");
    println!("---\n");

    // 初始化按键配置
    init_keys(&[0x4C, 0x4B, 0x4A]);  // L, K, J

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

    SHOULD_EXIT.store(true, Ordering::SeqCst);
    let _ = autofire_thread.join();

    println!("程序已退出");
}

fn init_keys(vk_list: &[u16]) {
    let mut states = KEY_STATES.write();
    for &vk in vk_list {
        let sc = unsafe { MapVirtualKeyW(vk as u32, MAPVK_VK_TO_VSC) as u16 };
        println!("注册按键: vk={:#04X}, sc={:#04X}", vk, sc);
        states.insert(vk as u32, KeyState {
            vk,
            sc,
            pressed: false,
        });
    }
}

fn get_key_name(vk: u32) -> &'static str {
    match vk {
        0x4A => "J",
        0x4B => "K",
        0x4C => "L",
        _ => "?",
    }
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
        let should_handle = {
            let states = KEY_STATES.read();
            states.contains_key(&vk)
        };

        if should_handle && !is_injected {
            if is_keydown {
                let mut states = KEY_STATES.write();
                if let Some(state) = states.get_mut(&vk) {
                    if !state.pressed {
                        // 首次按下
                        println!("[Hook] {} 按下", get_key_name(vk));
                        state.pressed = true;

                        // 发送一次正常按键（聊天框能识别）
                        send_key_normal(state.vk, state.sc, false);
                    }
                }
                // 拦截所有原始按键
                return LRESULT(1);
            } else if is_keyup {
                let mut states = KEY_STATES.write();
                if let Some(state) = states.get_mut(&vk) {
                    println!("[Hook] {} 释放", get_key_name(vk));
                    state.pressed = false;

                    // 发送正常释放
                    send_key_normal(state.vk, state.sc, true);
                }
                // 拦截原始释放
                return LRESULT(1);
            }
        }
    }

    CallNextHookEx(None, code, wparam, lparam)
}

fn autofire_loop() {
    println!("[AutoFire] 连发线程已启动");

    // 设置线程优先级
    unsafe {
        let thread = GetCurrentThread();
        let _ = SetThreadPriority(thread, THREAD_PRIORITY_ABOVE_NORMAL);
    }

    let mut send_count: u64 = 0;

    loop {
        if SHOULD_EXIT.load(Ordering::SeqCst) {
            break;
        }

        // 获取当前按下的按键
        let pressed_keys: Vec<(u16, u16)> = {
            let states = KEY_STATES.read();
            states.values()
                .filter(|s| s.pressed)
                .map(|s| (s.vk, s.sc))
                .collect()
        };

        if !pressed_keys.is_empty() {
            // 为每个按下的键发送按键
            for &(_vk, sc) in &pressed_keys {
                send_key_game_only(sc, false);
            }

            thread::sleep(Duration::from_millis(KEY_DOWN_MS));

            for &(_vk, sc) in &pressed_keys {
                send_key_game_only(sc, true);
            }

            thread::sleep(Duration::from_millis(KEY_UP_MS));

            send_count += 1;
            if send_count % 30 == 0 {
                println!("[AutoFire] 已发送 {} 轮，当前按键数: {}", send_count, pressed_keys.len());
            }
        } else {
            send_count = 0;
            thread::sleep(Duration::from_millis(5));
        }
    }

    println!("[AutoFire] 连发线程已停止");
}

/// 发送正常按键（聊天框能识别）
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

/// 发送游戏专用按键（游戏识别，聊天框不识别）
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
