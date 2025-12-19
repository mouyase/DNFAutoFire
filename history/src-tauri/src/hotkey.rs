//! 全局快捷键模块（基于键盘钩子）
//!
//! 使用低级键盘钩子实现全局快捷键，不会拦截按键
//! 只在 DNF 窗口激活时响应快捷键
//! 支持动态修改快捷键配置

use parking_lot::RwLock;
use std::sync::atomic::{AtomicBool, Ordering};
use std::thread;
use tauri::{AppHandle, Emitter, Manager, Runtime};

/// 默认唤醒快捷键：Alt+`（打开迷你窗口）
pub const DEFAULT_POPUP_SHORTCUT: &str = "Alt+`";

/// 默认开关快捷键：Pause（开启/停止连发）
pub const DEFAULT_TOGGLE_SHORTCUT: &str = "Pause";

/// 快捷键配置
#[derive(Debug, Clone)]
pub struct HotkeyConfig {
    /// 唤醒弹窗快捷键
    pub popup_shortcut: ParsedShortcut,
    /// 开关连发快捷键
    pub toggle_shortcut: ParsedShortcut,
}

impl Default for HotkeyConfig {
    fn default() -> Self {
        Self {
            popup_shortcut: parse_shortcut(DEFAULT_POPUP_SHORTCUT),
            toggle_shortcut: parse_shortcut(DEFAULT_TOGGLE_SHORTCUT),
        }
    }
}

/// 解析后的快捷键
#[derive(Debug, Clone, Default)]
pub struct ParsedShortcut {
    /// 是否需要 Ctrl
    pub ctrl: bool,
    /// 是否需要 Alt
    pub alt: bool,
    /// 是否需要 Shift
    pub shift: bool,
    /// 主键的 VK 码
    pub vk: u32,
}

// 全局状态
static HOOK_RUNNING: AtomicBool = AtomicBool::new(false);

// 修饰键状态
static CTRL_PRESSED: AtomicBool = AtomicBool::new(false);
static ALT_PRESSED: AtomicBool = AtomicBool::new(false);
static SHIFT_PRESSED: AtomicBool = AtomicBool::new(false);

// 快捷键触发标志
static POPUP_TRIGGERED: AtomicBool = AtomicBool::new(false);
static TOGGLE_TRIGGERED: AtomicBool = AtomicBool::new(false);

// 全局快捷键配置
static HOTKEY_CONFIG: once_cell::sync::Lazy<RwLock<HotkeyConfig>> =
    once_cell::sync::Lazy::new(|| RwLock::new(HotkeyConfig::default()));

/// 更新快捷键配置
pub fn update_hotkey_config(popup: &str, toggle: &str) {
    let mut config = HOTKEY_CONFIG.write();
    config.popup_shortcut = parse_shortcut(popup);
    config.toggle_shortcut = parse_shortcut(toggle);
    println!(
        "[快捷键] 配置已更新: popup={}, toggle={}",
        popup, toggle
    );
}

/// 获取当前快捷键配置
pub fn get_hotkey_config() -> HotkeyConfig {
    HOTKEY_CONFIG.read().clone()
}

/// 解析快捷键字符串为结构体
/// 支持格式：Alt+`, Ctrl+Shift+A, Pause 等
pub fn parse_shortcut(shortcut: &str) -> ParsedShortcut {
    let mut result = ParsedShortcut::default();
    let parts: Vec<&str> = shortcut.split('+').collect();

    for part in parts {
        let part = part.trim();
        match part.to_lowercase().as_str() {
            "ctrl" | "control" => result.ctrl = true,
            "alt" => result.alt = true,
            "shift" => result.shift = true,
            _ => {
                // 主键
                result.vk = key_to_vk(part);
            }
        }
    }

    result
}

/// 将键名转换为 VK 码
fn key_to_vk(key: &str) -> u32 {
    match key.to_uppercase().as_str() {
        // 特殊键
        "`" => 0xC0,           // VK_OEM_3
        "PAUSE" => 0x13,       // VK_PAUSE
        "ESC" | "ESCAPE" => 0x1B,
        "SPACE" => 0x20,
        "TAB" => 0x09,
        "ENTER" | "RETURN" => 0x0D,
        "BACKSPACE" => 0x08,
        "DELETE" | "DEL" => 0x2E,
        "INSERT" | "INS" => 0x2D,
        "HOME" => 0x24,
        "END" => 0x23,
        "PAGEUP" | "PGUP" => 0x21,
        "PAGEDOWN" | "PGDN" => 0x22,
        "UP" => 0x26,
        "DOWN" => 0x28,
        "LEFT" => 0x25,
        "RIGHT" => 0x27,
        "CAPSLOCK" => 0x14,
        "NUMLOCK" => 0x90,
        "SCROLLLOCK" => 0x91,
        "PRINTSCREEN" | "PRTSC" => 0x2C,

        // 功能键
        "F1" => 0x70,
        "F2" => 0x71,
        "F3" => 0x72,
        "F4" => 0x73,
        "F5" => 0x74,
        "F6" => 0x75,
        "F7" => 0x76,
        "F8" => 0x77,
        "F9" => 0x78,
        "F10" => 0x79,
        "F11" => 0x7A,
        "F12" => 0x7B,

        // 数字键
        "0" => 0x30,
        "1" => 0x31,
        "2" => 0x32,
        "3" => 0x33,
        "4" => 0x34,
        "5" => 0x35,
        "6" => 0x36,
        "7" => 0x37,
        "8" => 0x38,
        "9" => 0x39,

        // 字母键
        "A" => 0x41,
        "B" => 0x42,
        "C" => 0x43,
        "D" => 0x44,
        "E" => 0x45,
        "F" => 0x46,
        "G" => 0x47,
        "H" => 0x48,
        "I" => 0x49,
        "J" => 0x4A,
        "K" => 0x4B,
        "L" => 0x4C,
        "M" => 0x4D,
        "N" => 0x4E,
        "O" => 0x4F,
        "P" => 0x50,
        "Q" => 0x51,
        "R" => 0x52,
        "S" => 0x53,
        "T" => 0x54,
        "U" => 0x55,
        "V" => 0x56,
        "W" => 0x57,
        "X" => 0x58,
        "Y" => 0x59,
        "Z" => 0x5A,

        // 符号键
        "-" | "MINUS" => 0xBD,
        "=" | "EQUALS" => 0xBB,
        "[" | "LBRACKET" => 0xDB,
        "]" | "RBRACKET" => 0xDD,
        "\\" | "BACKSLASH" => 0xDC,
        ";" | "SEMICOLON" => 0xBA,
        "'" | "QUOTE" => 0xDE,
        "," | "COMMA" => 0xBC,
        "." | "PERIOD" => 0xBE,
        "/" | "SLASH" => 0xBF,

        // 默认返回 0
        _ => 0,
    }
}

/// 启动快捷键钩子
pub fn start_hotkey_hook<R: Runtime>(app: &AppHandle<R>) {
    if HOOK_RUNNING.load(Ordering::SeqCst) {
        println!("[快捷键] 钩子已在运行");
        return;
    }

    let app_handle = app.clone();

    // 启动钩子线程
    thread::spawn(|| {
        run_keyboard_hook();
    });

    // 启动事件处理线程
    thread::spawn(move || {
        run_event_handler(app_handle);
    });
}

/// 事件处理循环
fn run_event_handler<R: Runtime>(app: AppHandle<R>) {
    // 等待钩子启动
    while !HOOK_RUNNING.load(Ordering::SeqCst) {
        thread::sleep(std::time::Duration::from_millis(10));
    }

    loop {
        // 检查 popup 触发
        if POPUP_TRIGGERED.swap(false, Ordering::SeqCst) {
            handle_popup_shortcut(&app);
        }

        // 检查 toggle 触发
        if TOGGLE_TRIGGERED.swap(false, Ordering::SeqCst) {
            handle_toggle_shortcut(&app);
        }

        thread::sleep(std::time::Duration::from_millis(5));
    }
}

#[cfg(windows)]
fn run_keyboard_hook() {
    use crate::core::traits::WindowDetector;
    use crate::core::window::DefaultWindowDetector;
    use windows::Win32::Foundation::{LPARAM, LRESULT, WPARAM};
    use windows::Win32::System::LibraryLoader::GetModuleHandleW;
    use windows::Win32::UI::WindowsAndMessaging::{
        CallNextHookEx, GetMessageW, SetWindowsHookExW, UnhookWindowsHookEx, KBDLLHOOKSTRUCT, MSG,
        WH_KEYBOARD_LL,
    };

    // 全局窗口检测器
    static DETECTOR: once_cell::sync::Lazy<DefaultWindowDetector> =
        once_cell::sync::Lazy::new(DefaultWindowDetector::new);

    unsafe extern "system" fn keyboard_proc(
        code: i32,
        wparam: WPARAM,
        lparam: LPARAM,
    ) -> LRESULT {
        // VK 码定义
        const VK_CONTROL: u32 = 0x11;
        const VK_LCONTROL: u32 = 0xA2;
        const VK_RCONTROL: u32 = 0xA3;
        const VK_MENU: u32 = 0x12;
        const VK_LMENU: u32 = 0xA4;
        const VK_RMENU: u32 = 0xA5;
        const VK_SHIFT: u32 = 0x10;
        const VK_LSHIFT: u32 = 0xA0;
        const VK_RSHIFT: u32 = 0xA1;
        const WM_KEYDOWN: u32 = 0x0100;
        const WM_SYSKEYDOWN: u32 = 0x0104;
        const WM_KEYUP: u32 = 0x0101;
        const WM_SYSKEYUP: u32 = 0x0105;

        if code >= 0 {
            let kb = *(lparam.0 as *const KBDLLHOOKSTRUCT);
            let vk = kb.vkCode;
            let msg = wparam.0 as u32;

            // 跟踪修饰键状态
            if vk == VK_CONTROL || vk == VK_LCONTROL || vk == VK_RCONTROL {
                if msg == WM_KEYDOWN || msg == WM_SYSKEYDOWN {
                    CTRL_PRESSED.store(true, Ordering::SeqCst);
                } else if msg == WM_KEYUP || msg == WM_SYSKEYUP {
                    CTRL_PRESSED.store(false, Ordering::SeqCst);
                }
            }
            if vk == VK_MENU || vk == VK_LMENU || vk == VK_RMENU {
                if msg == WM_KEYDOWN || msg == WM_SYSKEYDOWN {
                    ALT_PRESSED.store(true, Ordering::SeqCst);
                } else if msg == WM_KEYUP || msg == WM_SYSKEYUP {
                    ALT_PRESSED.store(false, Ordering::SeqCst);
                }
            }
            if vk == VK_SHIFT || vk == VK_LSHIFT || vk == VK_RSHIFT {
                if msg == WM_KEYDOWN || msg == WM_SYSKEYDOWN {
                    SHIFT_PRESSED.store(true, Ordering::SeqCst);
                } else if msg == WM_KEYUP || msg == WM_SYSKEYUP {
                    SHIFT_PRESSED.store(false, Ordering::SeqCst);
                }
            }

            // 检测快捷键（只在按下时触发）
            let is_keydown = msg == WM_KEYDOWN || msg == WM_SYSKEYDOWN;
            if is_keydown {
                let ctrl_pressed = CTRL_PRESSED.load(Ordering::SeqCst);
                let alt_pressed = ALT_PRESSED.load(Ordering::SeqCst);
                let shift_pressed = SHIFT_PRESSED.load(Ordering::SeqCst);

                // 检查 DNF 窗口是否激活
                static DETECTOR: once_cell::sync::Lazy<
                    crate::core::window::DefaultWindowDetector,
                > = once_cell::sync::Lazy::new(crate::core::window::DefaultWindowDetector::new);

                let is_dnf_active = DETECTOR.is_target_active();

                if is_dnf_active {
                    // 读取当前配置
                    let config = HOTKEY_CONFIG.read();

                    // 检查唤醒弹窗快捷键
                    let popup = &config.popup_shortcut;
                    if popup.vk != 0
                        && vk == popup.vk
                        && ctrl_pressed == popup.ctrl
                        && alt_pressed == popup.alt
                        && shift_pressed == popup.shift
                    {
                        println!("[快捷键] 唤醒弹窗触发 (DNF 窗口激活)");
                        POPUP_TRIGGERED.store(true, Ordering::SeqCst);
                    }

                    // 检查开关连发快捷键
                    let toggle = &config.toggle_shortcut;
                    if toggle.vk != 0
                        && vk == toggle.vk
                        && ctrl_pressed == toggle.ctrl
                        && alt_pressed == toggle.alt
                        && shift_pressed == toggle.shift
                    {
                        println!("[快捷键] 开关连发触发 (DNF 窗口激活)");
                        TOGGLE_TRIGGERED.store(true, Ordering::SeqCst);
                    }
                }
            }
        }

        // 始终传递按键，不拦截
        CallNextHookEx(None, code, wparam, lparam)
    }

    unsafe {
        // 预热 DETECTOR
        let _ = DETECTOR.is_target_active();

        HOOK_RUNNING.store(true, Ordering::SeqCst);

        let hook = SetWindowsHookExW(
            WH_KEYBOARD_LL,
            Some(keyboard_proc),
            GetModuleHandleW(None).unwrap_or_default(),
            0,
        );

        if let Ok(hook) = hook {
            println!("[快捷键] 键盘钩子已安装");

            let mut msg = MSG::default();
            loop {
                if GetMessageW(&mut msg, None, 0, 0).as_bool() {
                    // 处理消息
                } else {
                    break;
                }
            }

            let _ = UnhookWindowsHookEx(hook);
            println!("[快捷键] 键盘钩子已卸载");
        } else {
            eprintln!("[快捷键] 安装键盘钩子失败");
        }

        HOOK_RUNNING.store(false, Ordering::SeqCst);
    }
}

#[cfg(not(windows))]
fn run_keyboard_hook() {
    println!("[快捷键] 非 Windows 平台，跳过钩子安装");
    HOOK_RUNNING.store(true, Ordering::SeqCst);
}

/// 处理唤醒快捷键
fn handle_popup_shortcut<R: Runtime>(app: &AppHandle<R>) {
    if let Some(window) = app.get_webview_window("mini") {
        if window.is_visible().unwrap_or(false) {
            // 窗口已显示，发送选择下一个配置的事件
            let _ = window.emit("select-next-profile", ());
            println!("[快捷键] 选择下一个配置");
        } else {
            // 显示窗口
            let _ = window.show();
            let _ = window.set_focus();
            let _ = window.set_always_on_top(true);
            let _ = window.center();
            println!("[快捷键] 显示迷你窗口");
        }
    } else {
        println!("[快捷键] 迷你窗口不存在");
    }
}

/// 处理开关快捷键
fn handle_toggle_shortcut<R: Runtime>(app: &AppHandle<R>) {
    if let Some(window) = app.get_webview_window("main") {
        let _ = window.emit("tray-toggle-autofire", ());
        println!("[快捷键] 发送连发切换事件");
    }
}

/// 停止快捷键钩子
#[allow(dead_code)]
pub fn stop_hotkey_hook() {
    HOOK_RUNNING.store(false, Ordering::SeqCst);
}
