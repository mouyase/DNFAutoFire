//! 最终方案测试（优化版）
//!
//! 使用发现的方式：vkFF + sc + 无 SCANCODE 标志
//! - 游戏能识别（读取 wScan）
//! - 聊天框不识别（wVk=0xFF 无效）
//!
//! 优化：
//! - 高精度定时器 (timeBeginPeriod)
//! - 高线程优先级
//! - 减少不必要的延迟
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
    KEYEVENTF_KEYUP, KEYEVENTF_SCANCODE, KEYBD_EVENT_FLAGS, VIRTUAL_KEY,
    MapVirtualKeyW, MAPVK_VK_TO_VSC,
};
use windows::Win32::System::LibraryLoader::GetModuleHandleW;
use windows::Win32::Media::{timeBeginPeriod, timeEndPeriod};
use windows::Win32::System::Threading::{
    GetCurrentThread, SetThreadPriority, THREAD_PRIORITY_HIGHEST,
};

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
    println!("=== 最终方案测试（优化版）===");
    println!("请以管理员身份运行！\n");
    println!("按住 L 键：");
    println!("  - 聊天框：只输入一个 L（首次按下）");
    println!("  - 游戏战斗：连发 L");
    println!("按 ESC：退出程序");
    println!("---\n");

    // 设置高精度定时器
    unsafe { timeBeginPeriod(1); }

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

    // 恢复定时器精度
    unsafe { timeEndPeriod(1); }

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

                    // 发送一次正常按键（聊天框能识别）
                    let vk = L_VK.load(Ordering::SeqCst);
                    let sc = L_SC.load(Ordering::SeqCst);
                    send_key_normal(vk, sc, false);
                }
                // 拦截所有原始按键（包括系统重复）
                return LRESULT(1);
            } else if is_keyup {
                println!("[Hook] L 释放 - 停止连发");
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

    // 设置线程优先级为最高
    unsafe {
        let thread = GetCurrentThread();
        let _ = SetThreadPriority(thread, THREAD_PRIORITY_HIGHEST);
    }

    let mut send_count: u64 = 0;

    while !SHOULD_EXIT.load(Ordering::SeqCst) {
        if L_KEY_DOWN.load(Ordering::SeqCst) {
            let sc = L_SC.load(Ordering::SeqCst);

            // 用 vkFF + sc + 无 SCANCODE 标志（游戏识别，聊天框不识别）
            send_key_game_only(sc, false);
            thread::sleep(Duration::from_millis(1));
            send_key_game_only(sc, true);
            thread::sleep(Duration::from_millis(1));

            send_count += 1;
            if send_count % 100 == 0 {
                println!("[AutoFire] 已发送 {} 次", send_count);
            }
        } else {
            send_count = 0;
            thread::sleep(Duration::from_millis(1));
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
/// 关键：vkFF + sc + 无 KEYEVENTF_SCANCODE 标志
fn send_key_game_only(sc: u16, key_up: bool) {
    let flags = if key_up {
        KEYEVENTF_KEYUP
    } else {
        KEYBD_EVENT_FLAGS(0)  // 无 SCANCODE 标志
    };

    let input = INPUT {
        r#type: INPUT_KEYBOARD,
        Anonymous: INPUT_0 {
            ki: KEYBDINPUT {
                wVk: VIRTUAL_KEY(0xFF),  // 无效虚拟键码
                wScan: sc,                // 真实扫描码
                dwFlags: flags,           // 无 SCANCODE 标志
                time: 0,
                dwExtraInfo: 0,
            },
        },
    };

    unsafe {
        SendInput(&[input], std::mem::size_of::<INPUT>() as i32);
    }
}
