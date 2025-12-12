//! 按住时间检测 - 短按正常，长按连发
//!
//! 思路：
//! - 短按 L（< 150ms）：正常输入一个 L（用于聊天）
//! - 长按 L（> 150ms）：连发 L（用于技能）
//!
//! 按 ESC 退出

use std::sync::atomic::{AtomicBool, AtomicU32, AtomicU64, Ordering};
use std::thread;
use std::time::{Duration, Instant};

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

const HOLD_THRESHOLD_MS: u64 = 150;  // 长按阈值

// 全局状态
static L_KEY_DOWN: AtomicBool = AtomicBool::new(false);
static L_LONG_PRESS: AtomicBool = AtomicBool::new(false);  // 是否进入长按模式
static L_PRESS_TIME: AtomicU64 = AtomicU64::new(0);  // 按下时间戳（ms）
static SHOULD_EXIT: AtomicBool = AtomicBool::new(false);
static L_SCAN_CODE: AtomicU32 = AtomicU32::new(0);

fn main() {
    println!("=== 长按检测测试 ===");
    println!("请以管理员身份运行！\n");
    println!("短按 L（< {}ms）：正常输入一个 L", HOLD_THRESHOLD_MS);
    println!("长按 L（> {}ms）：连发 L", HOLD_THRESHOLD_MS);
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

fn current_time_ms() -> u64 {
    use std::time::{SystemTime, UNIX_EPOCH};
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_millis() as u64
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
        if vk == VK_L && !is_injected {
            if is_keydown {
                if !L_KEY_DOWN.load(Ordering::SeqCst) {
                    // 首次按下，记录时间
                    let now = current_time_ms();
                    L_PRESS_TIME.store(now, Ordering::SeqCst);
                    L_KEY_DOWN.store(true, Ordering::SeqCst);
                    L_LONG_PRESS.store(false, Ordering::SeqCst);
                    println!("[Hook] L 按下 - 等待判断长短按...");
                    // 暂时拦截，让连发线程判断
                    return LRESULT(1);
                } else {
                    // 系统重复键
                    if L_LONG_PRESS.load(Ordering::SeqCst) {
                        // 已进入长按模式，拦截
                        return LRESULT(1);
                    } else {
                        // 还在等待判断，检查时间
                        let press_time = L_PRESS_TIME.load(Ordering::SeqCst);
                        let now = current_time_ms();
                        if now - press_time > HOLD_THRESHOLD_MS {
                            // 超过阈值，进入长按模式
                            println!("[Hook] 进入长按模式");
                            L_LONG_PRESS.store(true, Ordering::SeqCst);
                        }
                        // 拦截系统重复
                        return LRESULT(1);
                    }
                }
            } else if is_keyup {
                let was_long_press = L_LONG_PRESS.load(Ordering::SeqCst);
                let press_time = L_PRESS_TIME.load(Ordering::SeqCst);
                let now = current_time_ms();
                let hold_duration = now - press_time;

                L_KEY_DOWN.store(false, Ordering::SeqCst);
                L_LONG_PRESS.store(false, Ordering::SeqCst);

                if !was_long_press && hold_duration < HOLD_THRESHOLD_MS {
                    // 短按，发送一次按键
                    println!("[Hook] 短按 {}ms - 发送一次 L", hold_duration);
                    let scan_code = L_SCAN_CODE.load(Ordering::SeqCst) as u16;
                    send_key(scan_code, false);
                    thread::sleep(Duration::from_millis(10));
                    send_key(scan_code, true);
                } else {
                    println!("[Hook] 长按 {}ms - 停止连发", hold_duration);
                }

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
        // 只有进入长按模式才连发
        if L_LONG_PRESS.load(Ordering::SeqCst) {
            let scan_code = L_SCAN_CODE.load(Ordering::SeqCst) as u16;

            send_key(scan_code, false);
            thread::sleep(Duration::from_millis(16));
            send_key(scan_code, true);
            thread::sleep(Duration::from_millis(17));

            send_count += 1;
            if send_count % 30 == 0 {
                println!("[AutoFire] 已发送 {} 次", send_count);
            }
        } else {
            // 检查是否应该进入长按模式
            if L_KEY_DOWN.load(Ordering::SeqCst) && !L_LONG_PRESS.load(Ordering::SeqCst) {
                let press_time = L_PRESS_TIME.load(Ordering::SeqCst);
                let now = current_time_ms();
                if now - press_time > HOLD_THRESHOLD_MS {
                    println!("[AutoFire] 检测到长按，开始连发");
                    L_LONG_PRESS.store(true, Ordering::SeqCst);
                }
            }
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
