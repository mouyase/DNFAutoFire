//! 连发核心逻辑
//!
//! 基于键盘钩子和多线程实现的连发引擎

use parking_lot::RwLock;
use std::collections::HashSet;
use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};
use std::sync::Arc;

use super::keyboard::WindowsKeyboardDriver;
use super::window::{WindowDetector, WindowsWindowDetector};

/// 连发引擎
pub struct AutoFireEngine {
    /// 是否启用连发
    enabled: Arc<AtomicBool>,
    /// 配置的连发按键集合（虚拟键码）
    configured_keys: Arc<RwLock<HashSet<u16>>>,
    /// 当前物理按下的按键（位掩码）
    pressed_keys: Arc<AtomicU64>,
    /// 平台实现
    #[cfg(windows)]
    platform: windows_impl::WindowsAutoFire,
}

impl std::fmt::Debug for AutoFireEngine {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("AutoFireEngine")
            .field("enabled", &self.enabled.load(Ordering::SeqCst))
            .field("configured_keys", &*self.configured_keys.read())
            .finish()
    }
}

impl AutoFireEngine {
    pub fn new() -> Self {
        Self {
            enabled: Arc::new(AtomicBool::new(false)),
            configured_keys: Arc::new(RwLock::new(HashSet::new())),
            pressed_keys: Arc::new(AtomicU64::new(0)),
            #[cfg(windows)]
            platform: windows_impl::WindowsAutoFire::new(),
        }
    }

    /// 设置连发按键列表
    pub fn set_keys(&self, keys: Vec<u16>) {
        println!("[AutoFire] 设置连发按键: {:?}", keys);
        let mut configured = self.configured_keys.write();
        configured.clear();
        for vk in keys {
            configured.insert(vk);
        }
    }

    /// 启动连发引擎
    pub fn start(&mut self) {
        if self.enabled.load(Ordering::SeqCst) {
            println!("[AutoFire] 引擎已在运行中");
            return;
        }

        println!("[AutoFire] 启动连发引擎...");
        self.enabled.store(true, Ordering::SeqCst);
        self.pressed_keys.store(0, Ordering::SeqCst);

        self.platform.start(
            self.enabled.clone(),
            self.configured_keys.clone(),
            self.pressed_keys.clone(),
        );
    }

    /// 停止连发引擎
    pub fn stop(&mut self) {
        println!("[AutoFire] 停止连发引擎...");
        self.enabled.store(false, Ordering::SeqCst);
        self.platform.stop();
    }

    /// 检查引擎是否运行中
    #[allow(dead_code)]
    pub fn is_running(&self) -> bool {
        self.enabled.load(Ordering::SeqCst)
    }
}

impl Default for AutoFireEngine {
    fn default() -> Self {
        Self::new()
    }
}

impl Drop for AutoFireEngine {
    fn drop(&mut self) {
        self.stop();
    }
}

// ============================================================================
// Windows 实现
// ============================================================================

#[cfg(windows)]
mod windows_impl {
    use super::*;
    use std::thread::{self, JoinHandle};
    use std::time::Duration;

    use super::super::keyboard::KeyboardDriver;

    /// 连发时间参数（毫秒）
    const KEY_DOWN_MS: u64 = 16;
    const KEY_UP_MS: u64 = 17;

    pub struct WindowsAutoFire {
        /// 连发线程句柄
        thread_handle: Option<JoinHandle<()>>,
        /// 钩子线程句柄
        hook_thread_handle: Option<JoinHandle<()>>,
        /// 停止信号
        stop_signal: Arc<AtomicBool>,
    }

    impl WindowsAutoFire {
        pub fn new() -> Self {
            Self {
                thread_handle: None,
                hook_thread_handle: None,
                stop_signal: Arc::new(AtomicBool::new(false)),
            }
        }

        pub fn start(
            &mut self,
            enabled: Arc<AtomicBool>,
            configured_keys: Arc<RwLock<HashSet<u16>>>,
            pressed_keys: Arc<AtomicU64>,
        ) {
            if self.thread_handle.is_some() {
                return;
            }

            self.stop_signal.store(false, Ordering::SeqCst);

            // 设置高精度定时器
            set_timer_resolution(true);

            // 启动钩子线程
            let hook_enabled = enabled.clone();
            let hook_configured_keys = configured_keys.clone();
            let hook_pressed_keys = pressed_keys.clone();
            let hook_stop_signal = self.stop_signal.clone();

            let hook_handle = thread::spawn(move || {
                run_keyboard_hook(
                    hook_enabled,
                    hook_configured_keys,
                    hook_pressed_keys,
                    hook_stop_signal,
                );
            });
            self.hook_thread_handle = Some(hook_handle);

            // 启动连发线程
            let fire_enabled = enabled;
            let fire_configured_keys = configured_keys;
            let fire_pressed_keys = pressed_keys;
            let fire_stop_signal = self.stop_signal.clone();

            let handle = thread::spawn(move || {
                autofire_loop(
                    fire_enabled,
                    fire_configured_keys,
                    fire_pressed_keys,
                    fire_stop_signal,
                );
            });

            self.thread_handle = Some(handle);
        }

        pub fn stop(&mut self) {
            self.stop_signal.store(true, Ordering::SeqCst);

            // 发送消息让钩子线程退出
            unsafe {
                let _ = windows::Win32::UI::WindowsAndMessaging::PostThreadMessageW(
                    get_hook_thread_id(),
                    windows::Win32::UI::WindowsAndMessaging::WM_QUIT,
                    windows::Win32::Foundation::WPARAM(0),
                    windows::Win32::Foundation::LPARAM(0),
                );
            }

            if let Some(handle) = self.thread_handle.take() {
                let _ = handle.join();
            }

            if let Some(handle) = self.hook_thread_handle.take() {
                let _ = handle.join();
            }

            set_timer_resolution(false);
        }
    }

    // 钩子线程 ID（用于发送退出消息）
    static HOOK_THREAD_ID: AtomicU64 = AtomicU64::new(0);

    fn get_hook_thread_id() -> u32 {
        HOOK_THREAD_ID.load(Ordering::SeqCst) as u32
    }

    /// 运行键盘钩子
    fn run_keyboard_hook(
        enabled: Arc<AtomicBool>,
        configured_keys: Arc<RwLock<HashSet<u16>>>,
        pressed_keys: Arc<AtomicU64>,
        stop_signal: Arc<AtomicBool>,
    ) {
        use windows::Win32::System::LibraryLoader::GetModuleHandleW;
        use windows::Win32::System::Threading::GetCurrentThreadId;
        use windows::Win32::UI::WindowsAndMessaging::{
            GetMessageW, SetWindowsHookExW, UnhookWindowsHookEx, MSG, WH_KEYBOARD_LL,
        };

        // 保存线程 ID
        unsafe {
            HOOK_THREAD_ID.store(GetCurrentThreadId() as u64, Ordering::SeqCst);
        }

        // 设置全局变量供钩子回调使用
        HOOK_ENABLED.store(true, Ordering::SeqCst);
        {
            let mut keys = HOOK_CONFIGURED_KEYS.write();
            *keys = configured_keys.read().clone();
        }

        // 存储共享状态的引用
        *HOOK_PRESSED_KEYS.write() = Some(pressed_keys.clone());
        *HOOK_ENABLED_FLAG.write() = Some(enabled.clone());
        *HOOK_CONFIGURED_KEYS_REF.write() = Some(configured_keys.clone());

        unsafe {
            let hmod = GetModuleHandleW(None)
                .ok()
                .map(|h| windows::Win32::Foundation::HINSTANCE(h.0));
            let hook = SetWindowsHookExW(WH_KEYBOARD_LL, Some(keyboard_hook_proc), hmod, 0);

            if let Ok(hook) = hook {
                println!("[AutoFire] 键盘钩子已安装");

                let mut msg = MSG::default();
                while !stop_signal.load(Ordering::SeqCst) {
                    if GetMessageW(&mut msg, None, 0, 0).as_bool() {
                        // 处理消息
                    } else {
                        break;
                    }
                }

                let _ = UnhookWindowsHookEx(hook);
                println!("[AutoFire] 键盘钩子已卸载");
            } else {
                eprintln!("[AutoFire] 安装键盘钩子失败");
            }
        }

        HOOK_ENABLED.store(false, Ordering::SeqCst);
    }

    // 钩子回调需要的全局状态
    static HOOK_ENABLED: AtomicBool = AtomicBool::new(false);
    static HOOK_CONFIGURED_KEYS: once_cell::sync::Lazy<RwLock<HashSet<u16>>> =
        once_cell::sync::Lazy::new(|| RwLock::new(HashSet::new()));
    static HOOK_PRESSED_KEYS: once_cell::sync::Lazy<RwLock<Option<Arc<AtomicU64>>>> =
        once_cell::sync::Lazy::new(|| RwLock::new(None));
    static HOOK_ENABLED_FLAG: once_cell::sync::Lazy<RwLock<Option<Arc<AtomicBool>>>> =
        once_cell::sync::Lazy::new(|| RwLock::new(None));
    #[allow(clippy::type_complexity)]
    static HOOK_CONFIGURED_KEYS_REF: once_cell::sync::Lazy<
        RwLock<Option<Arc<RwLock<HashSet<u16>>>>>,
    > = once_cell::sync::Lazy::new(|| RwLock::new(None));

    const WM_KEYDOWN: u32 = 0x0100;
    const WM_KEYUP: u32 = 0x0101;

    /// 将 VK 码转换为位掩码索引
    fn vk_to_bit(vk: u16) -> u64 {
        if vk < 64 {
            1u64 << vk
        } else {
            1u64 << (vk % 64)
        }
    }

    /// 键盘钩子回调
    unsafe extern "system" fn keyboard_hook_proc(
        code: i32,
        wparam: windows::Win32::Foundation::WPARAM,
        lparam: windows::Win32::Foundation::LPARAM,
    ) -> windows::Win32::Foundation::LRESULT {
        use windows::Win32::UI::WindowsAndMessaging::{
            CallNextHookEx, KBDLLHOOKSTRUCT, LLKHF_INJECTED, LLKHF_LOWER_IL_INJECTED,
        };

        if code >= 0 && HOOK_ENABLED.load(Ordering::SeqCst) {
            let kb = *(lparam.0 as *const KBDLLHOOKSTRUCT);
            let vk = kb.vkCode as u16;
            let is_keydown = wparam.0 as u32 == WM_KEYDOWN;
            let is_keyup = wparam.0 as u32 == WM_KEYUP;

            // 忽略注入的按键
            let is_injected = (kb.flags.0 & LLKHF_INJECTED.0) != 0
                || (kb.flags.0 & LLKHF_LOWER_IL_INJECTED.0) != 0;

            if !is_injected {
                let enabled_flag = HOOK_ENABLED_FLAG.read();
                let is_engine_enabled = enabled_flag
                    .as_ref()
                    .map(|f| f.load(Ordering::SeqCst))
                    .unwrap_or(false);

                if is_engine_enabled {
                    let configured_ref = HOOK_CONFIGURED_KEYS_REF.read();
                    let is_configured = configured_ref
                        .as_ref()
                        .map(|keys| keys.read().contains(&vk))
                        .unwrap_or(false);

                    if is_configured {
                        let pressed_keys = HOOK_PRESSED_KEYS.read();
                        if let Some(ref keys) = *pressed_keys {
                            let key_bit = vk_to_bit(vk);
                            let keyboard = WindowsKeyboardDriver::new();
                            let sc = keyboard.vk_to_scan_code(vk);

                            if is_keydown {
                                let old = keys.fetch_or(key_bit, Ordering::SeqCst);
                                if old & key_bit == 0 {
                                    println!("[Hook] 按键按下: vk={:#X}", vk);
                                    keyboard.send_key_down(vk, sc);
                                }
                                return windows::Win32::Foundation::LRESULT(1);
                            } else if is_keyup {
                                println!("[Hook] 按键释放: vk={:#X}", vk);
                                keys.fetch_and(!key_bit, Ordering::SeqCst);
                                keyboard.send_key_up(vk, sc);
                                return windows::Win32::Foundation::LRESULT(1);
                            }
                        }
                    }
                }
            }
        }

        CallNextHookEx(None, code, wparam, lparam)
    }

    /// 连发主循环
    fn autofire_loop(
        enabled: Arc<AtomicBool>,
        configured_keys: Arc<RwLock<HashSet<u16>>>,
        pressed_keys: Arc<AtomicU64>,
        stop_signal: Arc<AtomicBool>,
    ) {
        println!("[AutoFire] 连发线程已启动");

        set_thread_priority_high();

        let keyboard = WindowsKeyboardDriver::new();
        let window = WindowsWindowDetector::new();
        let mut keys_with_sc: Vec<(u16, u16, u64)> = Vec::new();

        loop {
            if stop_signal.load(Ordering::SeqCst) {
                break;
            }

            if !enabled.load(Ordering::SeqCst) {
                thread::sleep(Duration::from_millis(10));
                continue;
            }

            if !window.is_target_active() {
                static LAST_LOG: std::sync::atomic::AtomicU64 =
                    std::sync::atomic::AtomicU64::new(0);
                let now = std::time::SystemTime::now()
                    .duration_since(std::time::UNIX_EPOCH)
                    .unwrap()
                    .as_secs();
                let last = LAST_LOG.load(Ordering::Relaxed);
                if now - last >= 5 {
                    LAST_LOG.store(now, Ordering::Relaxed);
                    let class = window.get_foreground_class_name();
                    println!("[AutoFire] 等待目标窗口，当前: '{}'", class);
                }
                thread::sleep(Duration::from_millis(50));
                continue;
            }

            // 更新按键列表
            {
                let configured = configured_keys.read();
                if keys_with_sc.len() != configured.len() {
                    keys_with_sc.clear();
                    for &vk in configured.iter() {
                        let sc = keyboard.vk_to_scan_code(vk);
                        let bit = vk_to_bit(vk);
                        keys_with_sc.push((vk, sc, bit));
                    }
                }
            }

            let pressed = pressed_keys.load(Ordering::SeqCst);

            if pressed != 0 {
                for &(_vk, sc, bit) in &keys_with_sc {
                    if pressed & bit != 0 {
                        if stop_signal.load(Ordering::SeqCst) {
                            break;
                        }

                        // 使用游戏模式发送按键（VK=0xFF，只有游戏识别，聊天框不识别）
                        keyboard.send_game_key_down(sc);
                        thread::sleep(Duration::from_millis(KEY_DOWN_MS));
                        keyboard.send_game_key_up(sc);
                        thread::sleep(Duration::from_millis(KEY_UP_MS));
                    }
                }
            } else {
                thread::sleep(Duration::from_millis(5));
            }
        }

        println!("[AutoFire] 连发线程已停止");
    }

    /// 设置 Windows 定时器精度为 1ms
    fn set_timer_resolution(high_precision: bool) {
        use windows::Win32::Media::{timeBeginPeriod, timeEndPeriod};
        unsafe {
            if high_precision {
                timeBeginPeriod(1);
            } else {
                timeEndPeriod(1);
            }
        }
    }

    /// 设置当前线程为高优先级
    fn set_thread_priority_high() {
        use windows::Win32::System::Threading::{
            GetCurrentThread, SetThreadPriority, THREAD_PRIORITY_HIGHEST,
        };
        unsafe {
            let thread = GetCurrentThread();
            let _ = SetThreadPriority(thread, THREAD_PRIORITY_HIGHEST);
        }
    }
}
