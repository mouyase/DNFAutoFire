//! 钩子测试2 - 拦截系统重复键，只保留首次按下
//!
//! 思路：
//! - 第一次按下 L：放行，同时开始连发（vkFF+sc）
//! - 系统重复的 L：拦截（不放行）
//! - 释放 L：放行，停止连发
//!
//! 这样在聊天框里：只有第一个 L 会输入
//! 在游戏里：第一个 L + 连发的 L
//!
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
    KEYEVENTF_KEYUP, KEYEVENTF_SCANCODE, VIRTUAL_KEY,
    MapVirtualKeyW, MAPVK_VK_TO_VSC,
};
use windows::Win32::System::LibraryLoader::GetModuleHandleW;

const VK_L: u32 = 0x4C;
const VK_ESCAPE: u32 = 0x1B;
const WM_KEYDOWN: u32 = 0x0100;
const WM_KEYUP: u32 = 0x0101;

// 全局状态
static L_KEY_DOWN: AtomicBool = AtomicBool::new(false);
static L_FIRST_DOWN: AtomicBool = AtomicBool::new(false);  // 是否是第一次按下
static SHOULD_EXIT: AtomicBool = AtomicBool::new(false);
static L_SCAN_CODE: AtomicU32 = AtomicU32::new(0);

fn main() {
    println!("=== 钩子测试2 - 拦截系统重复键 ===");
    println!("请以管理员身份运行！\n");
    println!("按住 L 键：");
    println!("  - 第一次按下：放行（输入框会收到一个 L）");
    println!("  - 系统重复：拦截（输入框不会收到更多 L）");
    println!("  - 同时连发 vkFF+sc（游戏能感知）");
    println!("按 ESC：退出程序");
    println!("---\n");

    let l_scan = unsafe { MapVirtualKeyW(VK_L, MAPVK_VK_TO_VSC) };
    L_SCAN_CODE.store(l_scan, Ordering::SeqCst);
    println!("L 键 ScanCode: {:#04X}", l_scan);

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

        // L 键处理（只处理非注入的按键）
        if vk == VK_L && !is_injected {
            if is_keydown {
                if !L_KEY_DOWN.load(Ordering::SeqCst) {
                    // 第一次按下
                    println!("[Hook] L 首次按下 - 放行 + 开始连发");
                    L_KEY_DOWN.store(true, Ordering::SeqCst);
                    L_FIRST_DOWN.store(true, Ordering::SeqCst);
                    // 放行这次按键
                    return CallNextHookEx(None, code, wparam, lparam);
                } else {
                    // 系统重复键，拦截
                    println!("[Hook] L 重复按下 - 拦截");
                    return LRESULT(1);
                }
            } else if is_keyup {
                println!("[Hook] L 释放 - 放行 + 停止连发");
                L_KEY_DOWN.store(false, Ordering::SeqCst);
                L_FIRST_DOWN.store(false, Ordering::SeqCst);
                // 放行释放事件
                return CallNextHookEx(None, code, wparam, lparam);
            }
        }
    }

    CallNextHookEx(None, code, wparam, lparam)
}

fn autofire_loop() {
    println!("[AutoFire] 连发线程已启动");

    let mut send_count: u64 = 0;

    while !SHOULD_EXIT.load(Ordering::SeqCst) {
        if L_KEY_DOWN.load(Ordering::SeqCst) {
            // 等待第一次按下事件处理完毕
            if L_FIRST_DOWN.swap(false, Ordering::SeqCst) {
                // 第一次按下刚发生，稍等一下让原始按键先处理
                thread::sleep(Duration::from_millis(5));
            }

            let scan_code = L_SCAN_CODE.load(Ordering::SeqCst) as u16;

            // 用 vkFF + 真实sc 发送
            send_key(scan_code, false);
            thread::sleep(Duration::from_millis(16));
            send_key(scan_code, true);
            thread::sleep(Duration::from_millis(17));

            send_count += 1;
            if send_count % 30 == 0 {
                println!("[AutoFire] 已发送 {} 次", send_count);
            }
        } else {
            send_count = 0;
            thread::sleep(Duration::from_millis(5));
        }
    }

    println!("[AutoFire] 连发线程已停止");
}

fn send_key(scan_code: u16, key_up: bool) {
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
                wScan: scan_code,
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
