//! 连发核心逻辑
//!
//! 使用高精度定时器和多线程实现无冲突连发

use std::collections::HashMap;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::thread::{self, JoinHandle};
use std::time::{Duration, Instant};

use parking_lot::RwLock;

use super::keyboard::{self, vk_to_scan_code};
use super::window::is_dnf_active;

/// 连发引擎
pub struct AutoFireEngine {
    /// 是否启用连发
    enabled: Arc<AtomicBool>,
    /// 每个按键的连发状态
    key_states: Arc<RwLock<HashMap<u16, KeyState>>>,
    /// 连发线程句柄
    thread_handle: Option<JoinHandle<()>>,
    /// 停止信号
    stop_signal: Arc<AtomicBool>,
}

/// 单个按键的连发状态
#[derive(Clone)]
pub struct KeyState {
    /// 虚拟键码
    pub vk: u16,
    /// 扫描码
    pub scan_code: u16,
    /// 是否启用连发
    pub enabled: bool,
}

impl AutoFireEngine {
    pub fn new() -> Self {
        Self {
            enabled: Arc::new(AtomicBool::new(false)),
            key_states: Arc::new(RwLock::new(HashMap::new())),
            thread_handle: None,
            stop_signal: Arc::new(AtomicBool::new(false)),
        }
    }

    /// 添加一个按键到连发列表
    pub fn add_key(&self, vk: u16) {
        let scan_code = vk_to_scan_code(vk);
        let mut states = self.key_states.write();
        states.insert(vk, KeyState {
            vk,
            scan_code,
            enabled: true,
        });
    }

    /// 移除一个按键
    pub fn remove_key(&self, vk: u16) {
        let mut states = self.key_states.write();
        states.remove(&vk);
    }

    /// 切换按键的连发状态
    pub fn toggle_key(&self, vk: u16) -> bool {
        let mut states = self.key_states.write();
        if let Some(state) = states.get_mut(&vk) {
            state.enabled = !state.enabled;
            state.enabled
        } else {
            // 如果不存在，添加并启用
            let scan_code = vk_to_scan_code(vk);
            states.insert(vk, KeyState {
                vk,
                scan_code,
                enabled: true,
            });
            true
        }
    }

    /// 检查按键是否启用连发
    pub fn is_key_enabled(&self, vk: u16) -> bool {
        let states = self.key_states.read();
        states.get(&vk).map(|s| s.enabled).unwrap_or(false)
    }

    /// 获取所有启用的按键
    pub fn get_enabled_keys(&self) -> Vec<u16> {
        let states = self.key_states.read();
        states.iter()
            .filter(|(_, s)| s.enabled)
            .map(|(&vk, _)| vk)
            .collect()
    }

    /// 设置所有按键状态（用于加载配置）
    pub fn set_keys(&self, keys: Vec<u16>) {
        let mut states = self.key_states.write();
        states.clear();
        for vk in keys {
            let scan_code = vk_to_scan_code(vk);
            states.insert(vk, KeyState {
                vk,
                scan_code,
                enabled: true,
            });
        }
    }

    /// 启动连发引擎
    pub fn start(&mut self) {
        if self.thread_handle.is_some() {
            return; // 已经在运行
        }

        self.stop_signal.store(false, Ordering::SeqCst);
        self.enabled.store(true, Ordering::SeqCst);

        let enabled = self.enabled.clone();
        let key_states = self.key_states.clone();
        let stop_signal = self.stop_signal.clone();

        // 设置高精度定时器
        set_timer_resolution(true);

        let handle = thread::spawn(move || {
            autofire_loop(enabled, key_states, stop_signal);
        });

        self.thread_handle = Some(handle);
    }

    /// 停止连发引擎
    pub fn stop(&mut self) {
        self.enabled.store(false, Ordering::SeqCst);
        self.stop_signal.store(true, Ordering::SeqCst);

        if let Some(handle) = self.thread_handle.take() {
            let _ = handle.join();
        }

        // 恢复定时器精度
        set_timer_resolution(false);
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

/// 连发主循环
fn autofire_loop(
    enabled: Arc<AtomicBool>,
    key_states: Arc<RwLock<HashMap<u16, KeyState>>>,
    stop_signal: Arc<AtomicBool>,
) {
    println!("[AutoFire] 连发线程已启动");

    // 设置线程优先级为高
    set_thread_priority_high();

    let mut last_debug_time = Instant::now();
    let mut loop_count: u64 = 0;

    while !stop_signal.load(Ordering::SeqCst) {
        loop_count += 1;

        if !enabled.load(Ordering::SeqCst) {
            // 未启用时短暂休眠
            thread::sleep(Duration::from_millis(10));
            continue;
        }

        // 每秒打印一次调试信息
        if last_debug_time.elapsed() > Duration::from_secs(1) {
            let window_class = super::window::get_foreground_class_name();
            let is_dnf = is_dnf_active();
            let states_count = key_states.read().len();
            let enabled_count = key_states.read().values().filter(|s| s.enabled).count();

            println!("[AutoFire] 循环次数: {}, 窗口: '{}', 是DNF: {}, 按键数: {}/{}",
                loop_count, window_class, is_dnf, enabled_count, states_count);

            last_debug_time = Instant::now();
            loop_count = 0;
        }

        // 检查 DNF 窗口是否活动
        if !is_dnf_active() {
            thread::sleep(Duration::from_millis(50));
            continue;
        }

        // 获取当前需要连发的按键
        let states: Vec<KeyState> = {
            let guard = key_states.read();
            guard.values()
                .filter(|s| s.enabled)
                .cloned()
                .collect()
        };

        let mut any_key_pressed = false;

        // 检查每个按键
        for state in &states {
            if keyboard::is_key_pressed(state.vk) {
                // 按键被按下，发送连发（模仿 AHK 的方式）
                println!("[AutoFire] 发送按键: vk={:#X}, sc={:#X}", state.vk, state.scan_code);

                // 发送按下
                keyboard::send_key_down(state.scan_code);

                // 按下和释放之间的延迟（AHK 用 Sleep 1）
                thread::sleep(Duration::from_millis(1));

                // 发送释放
                keyboard::send_key_up(state.scan_code);

                // 两次按键之间的延迟
                thread::sleep(Duration::from_millis(1));

                any_key_pressed = true;
            }
        }

        // 没有按键按下时，短暂休眠降低 CPU 占用
        if !any_key_pressed {
            thread::sleep(Duration::from_millis(1));
        }
    }

    println!("[AutoFire] 连发线程已停止");
}

/// 高精度休眠（自旋等待）
/// 对于微秒级别的延迟，thread::sleep 不够精确
#[inline]
fn spin_sleep(duration: Duration) {
    let start = Instant::now();
    while start.elapsed() < duration {
        std::hint::spin_loop();
    }
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
