//! 键盘钩子测试 - 拦截 L 键，发送 L 键（vk=0xFF 方式）
//!
//! 测试输入法兼容性：
//! 按住 L 键：钩子拦截原始 L，用 vk=0xFF 方式发送 L
//! 在游戏聊天窗口测试：是否还会触发输入法？
//! 按 ESC：退出程序

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
static SHOULD_EXIT: AtomicBool = AtomicBool::new(false);
static L_SCAN_CODE: AtomicU32 = AtomicU32::new(0);

fn main() {
    println!("=== 输入法兼容性测试 ===");
    println!("请以管理员身份运行！\n");
    println!("按住 L 键：钩子拦截原始 L，用 vk=0xFF 方式发送 L");
    println!("测试：在游戏聊天窗口，是否还会触发输入法？");
    println!("按 ESC：退出程序");
    println!("---\n");

    // 获取 L 键的扫描码
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

        // 消息循环（必须有，否则钩子不工作）
        let mut msg = MSG::default();
        while !SHOULD_EXIT.load(Ordering::SeqCst) {
            if GetMessageW(&mut msg, None, 0, 0).as_bool() {
                // 处理消息
            }
        }

        // 卸载钩子
        let _ = UnhookWindowsHookEx(hook);
        println!("\n键盘钩子已卸载");
    }

    // 通知连发线程退出
    L_KEY_DOWN.store(false, Ordering::SeqCst);
    SHOULD_EXIT.store(true, Ordering::SeqCst);
    let _ = autofire_thread.join();

    println!("程序已退出");
}

/// 键盘钩子回调函数
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

        // 检查是否是我们自己注入的按键（避免无限循环）
        let is_injected = (kb.flags.0 & LLKHF_INJECTED.0) != 0
            || (kb.flags.0 & LLKHF_LOWER_IL_INJECTED.0) != 0;

        // ESC 键退出
        if vk == VK_ESCAPE && is_keydown {
            SHOULD_EXIT.store(true, Ordering::SeqCst);
            // 发送一个消息来唤醒 GetMessageW
            windows::Win32::UI::WindowsAndMessaging::PostQuitMessage(0);
        }

        // L 键处理
        if vk == VK_L && !is_injected {
            if is_keydown {
                if !L_KEY_DOWN.load(Ordering::SeqCst) {
                    println!("[Hook] L 键按下 - 开始连发");
                }
                L_KEY_DOWN.store(true, Ordering::SeqCst);
            } else if is_keyup {
                println!("[Hook] L 键释放 - 停止连发");
                L_KEY_DOWN.store(false, Ordering::SeqCst);
            }
            // 返回 1 表示拦截这个按键，不传递给系统
            return LRESULT(1);
        }
    }

    // 其他按键正常传递
    CallNextHookEx(None, code, wparam, lparam)
}

/// 连发线程
fn autofire_loop() {
    println!("[AutoFire] 连发线程已启动");

    let mut send_count: u64 = 0;

    while !SHOULD_EXIT.load(Ordering::SeqCst) {
        if L_KEY_DOWN.load(Ordering::SeqCst) {
            let scan_code = L_SCAN_CODE.load(Ordering::SeqCst) as u16;

            // 发送按键（vk=0xFF 方式）
            send_key(scan_code, false); // down
            thread::sleep(Duration::from_millis(16));
            send_key(scan_code, true);  // up
            thread::sleep(Duration::from_millis(17));

            send_count += 1;
            if send_count % 30 == 0 {
                println!("[AutoFire] 已发送 L 键 {} 次", send_count);
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
                wVk: VIRTUAL_KEY(0xFF), // vk=0xFF，输入法不识别
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
