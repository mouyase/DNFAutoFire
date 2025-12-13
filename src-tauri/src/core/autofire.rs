//! 连发核心逻辑
//!
//! Windows 平台: 使用低级键盘钩子 + 多线程实现无冲突连发
//! 其他平台: Mock 实现，用于 UI 开发测试
//!
//! 技术要点（Windows）：
//! 1. WH_KEYBOARD_LL 钩子拦截物理按键，阻止系统重复
//! 2. 首次按下发送正常按键（聊天框可识别）
//! 3. 后续连发使用 vkFF + sc 方式（仅游戏识别）
//! 4. 多键同时按下时，依次循环发送每个按键

use std::collections::HashSet;
use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};
use std::sync::Arc;

use parking_lot::RwLock;

// ============================================================================
// 公共接口 - AutoFireEngine
// ============================================================================

/// 连发引擎
pub struct AutoFireEngine {
    /// 是否启用连发
    enabled: Arc<AtomicBool>,
    /// 配置的连发按键集合（虚拟键码）
    configured_keys: Arc<RwLock<HashSet<u16>>>,
    /// 当前物理按下的按键（位掩码）
    pressed_keys: Arc<AtomicU64>,
    /// 平台特定实现
    #[cfg(windows)]
    platform: windows_impl::WindowsAutoFire,
    #[cfg(not(windows))]
    platform: mock_impl::MockAutoFire,
}

#[allow(dead_code)]
impl AutoFireEngine {
    pub fn new() -> Self {
        Self {
            enabled: Arc::new(AtomicBool::new(false)),
            configured_keys: Arc::new(RwLock::new(HashSet::new())),
            pressed_keys: Arc::new(AtomicU64::new(0)),
            #[cfg(windows)]
            platform: windows_impl::WindowsAutoFire::new(),
            #[cfg(not(windows))]
            platform: mock_impl::MockAutoFire::new(),
        }
    }

    /// 设置连发按键列表
    pub fn set_keys(&self, keys: Vec<u16>) {
        println!("[AutoFire] 设置按键: {:?}", keys);
        let mut configured = self.configured_keys.write();
        configured.clear();
        for vk in keys {
            configured.insert(vk);
        }
    }

    /// 获取所有启用的按键
    pub fn get_enabled_keys(&self) -> Vec<u16> {
        let configured = self.configured_keys.read();
        configured.iter().copied().collect()
    }

    /// 添加一个按键
    pub fn add_key(&self, vk: u16) {
        let mut configured = self.configured_keys.write();
        configured.insert(vk);
    }

    /// 移除一个按键
    pub fn remove_key(&self, vk: u16) {
        let mut configured = self.configured_keys.write();
        configured.remove(&vk);
    }

    /// 切换按键状态
    pub fn toggle_key(&self, vk: u16) -> bool {
        let mut configured = self.configured_keys.write();
        if configured.contains(&vk) {
            configured.remove(&vk);
            false
        } else {
            configured.insert(vk);
            true
        }
    }

    /// 检查按键是否启用
    pub fn is_key_enabled(&self, vk: u16) -> bool {
        let configured = self.configured_keys.read();
        configured.contains(&vk)
    }

    /// 启动连发引擎
    pub fn start(&mut self) {
        if self.enabled.load(Ordering::SeqCst) {
            return;
        }

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
        self.enabled.store(false, Ordering::SeqCst);
        self.platform.stop();
    }

    /// 检查引擎是否运行中
    pub fn is_running(&self) -> bool {
        self.enabled.load(Ordering::SeqCst)
    }

    /// 暂停/恢复连发
    pub fn toggle(&mut self) -> bool {
        if self.is_running() {
            self.stop();
            false
        } else {
            self.start();
            true
        }
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

    use crate::core::keyboard::{
        send_key_game_only_down, send_key_game_only_up, send_key_normal_down, send_key_normal_up,
        vk_to_scan_code,
    };
    use crate::core::window::is_dnf_active;

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
            GetMessageW, SetWindowsHookExW, UnhookWindowsHookEx, WH_KEYBOARD_LL, MSG,
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
            let hook = SetWindowsHookExW(
                WH_KEYBOARD_LL,
                Some(keyboard_hook_proc),
                GetModuleHandleW(None).unwrap_or_default(),
                0,
            );

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

                            if is_keydown {
                                let old = keys.fetch_or(key_bit, Ordering::SeqCst);
                                if old & key_bit == 0 {
                                    println!("[Hook] 按键按下: vk={:#X}", vk);
                                    let sc = vk_to_scan_code(vk);
                                    send_key_normal_down(vk, sc);
                                }
                                return windows::Win32::Foundation::LRESULT(1);
                            } else if is_keyup {
                                println!("[Hook] 按键释放: vk={:#X}", vk);
                                keys.fetch_and(!key_bit, Ordering::SeqCst);
                                let sc = vk_to_scan_code(vk);
                                send_key_normal_up(vk, sc);
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

        let mut keys_with_sc: Vec<(u16, u16, u64)> = Vec::new();

        loop {
            if stop_signal.load(Ordering::SeqCst) {
                break;
            }

            if !enabled.load(Ordering::SeqCst) {
                thread::sleep(Duration::from_millis(10));
                continue;
            }

            if !is_dnf_active() {
                static LAST_LOG: std::sync::atomic::AtomicU64 =
                    std::sync::atomic::AtomicU64::new(0);
                let now = std::time::SystemTime::now()
                    .duration_since(std::time::UNIX_EPOCH)
                    .unwrap()
                    .as_secs();
                let last = LAST_LOG.load(Ordering::Relaxed);
                if now - last >= 3 {
                    LAST_LOG.store(now, Ordering::Relaxed);
                    let class = crate::core::window::get_foreground_class_name();
                    println!("[AutoFire] 当前窗口: '{}' (非目标窗口)", class);
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
                        let sc = vk_to_scan_code(vk);
                        let bit = vk_to_bit(vk);
                        keys_with_sc.push((vk, sc, bit));
                    }
                }
            }

            let pressed = pressed_keys.load(Ordering::SeqCst);

            if pressed != 0 {
                for &(vk, sc, bit) in &keys_with_sc {
                    if pressed & bit != 0 {
                        if stop_signal.load(Ordering::SeqCst) {
                            break;
                        }

                        println!("[AutoFire] 发送按键 vk={:#X} sc={:#X}", vk, sc);

                        send_key_game_only_down(sc);
                        thread::sleep(Duration::from_millis(KEY_DOWN_MS));
                        send_key_game_only_up(sc);
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

// ============================================================================
// Mock 实现（非 Windows 平台，用于 UI 开发）
// ============================================================================

#[cfg(not(windows))]
mod mock_impl {
    use super::*;
    use std::thread::{self, JoinHandle};
    use std::time::Duration;

    pub struct MockAutoFire {
        /// 模拟连发线程句柄
        thread_handle: Option<JoinHandle<()>>,
        /// 停止信号
        stop_signal: Arc<AtomicBool>,
    }

    impl MockAutoFire {
        pub fn new() -> Self {
            Self {
                thread_handle: None,
                stop_signal: Arc::new(AtomicBool::new(false)),
            }
        }

        pub fn start(
            &mut self,
            enabled: Arc<AtomicBool>,
            configured_keys: Arc<RwLock<HashSet<u16>>>,
            _pressed_keys: Arc<AtomicU64>,
        ) {
            if self.thread_handle.is_some() {
                return;
            }

            self.stop_signal.store(false, Ordering::SeqCst);

            let stop_signal = self.stop_signal.clone();

            // Mock 模式下启动一个简单的监控线程
            let handle = thread::spawn(move || {
                println!("[AutoFire Mock] 连发引擎已启动（模拟模式）");

                while !stop_signal.load(Ordering::SeqCst) && enabled.load(Ordering::SeqCst) {
                    // 每秒打印一次状态
                    let keys = configured_keys.read();
                    if !keys.is_empty() {
                        println!(
                            "[AutoFire Mock] 监控中，已配置 {} 个按键",
                            keys.len()
                        );
                    }
                    thread::sleep(Duration::from_secs(1));
                }

                println!("[AutoFire Mock] 连发引擎已停止（模拟模式）");
            });

            self.thread_handle = Some(handle);
        }

        pub fn stop(&mut self) {
            self.stop_signal.store(true, Ordering::SeqCst);

            if let Some(handle) = self.thread_handle.take() {
                let _ = handle.join();
            }
        }
    }
}
