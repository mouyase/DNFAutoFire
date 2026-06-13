#![cfg_attr(not(windows), allow(dead_code))]

#[cfg(windows)]
use std::{
    env,
    mem::size_of,
    ptr::null_mut,
    sync::{
        atomic::{AtomicBool, AtomicU64, Ordering},
        Mutex,
    },
    thread,
    time::{Duration, Instant},
};

#[cfg(windows)]
use windows_sys::Win32::{
    Foundation::{GetLastError, HINSTANCE, HWND, LPARAM, LRESULT, WPARAM},
    System::LibraryLoader::GetModuleHandleW,
    UI::{
        Input::KeyboardAndMouse::{
            SendInput, INPUT, INPUT_0, INPUT_KEYBOARD, KEYBDINPUT, KEYEVENTF_KEYUP,
            KEYEVENTF_SCANCODE,
        },
        WindowsAndMessaging::{
            CallNextHookEx, DispatchMessageW, GetForegroundWindow, GetMessageW, GetWindowTextW,
            SetWindowsHookExW, TranslateMessage, UnhookWindowsHookEx, HC_ACTION, HHOOK,
            KBDLLHOOKSTRUCT, LLKHF_INJECTED, MSG, WH_KEYBOARD_LL, WM_KEYDOWN, WM_KEYUP,
            WM_SYSKEYDOWN, WM_SYSKEYUP,
        },
    },
};

#[cfg(windows)]
const VK_L: u32 = 0x4C;
#[cfg(windows)]
const VK_J: u32 = 0x4A;
#[cfg(windows)]
const VK_H: u32 = 0x48;
#[cfg(windows)]
const SCAN_L: u16 = 0x26;
#[cfg(windows)]
const SCAN_J: u16 = 0x24;
#[cfg(windows)]
const SCAN_H: u16 = 0x23;
#[cfg(windows)]
const EXTRA_INFO_MARKER: usize = 0xDFAF_0001;

#[cfg(windows)]
static PHYSICAL_L_HELD: AtomicBool = AtomicBool::new(false);
#[cfg(windows)]
static PHYSICAL_J_HELD: AtomicBool = AtomicBool::new(false);
#[cfg(windows)]
static PHYSICAL_H_HELD: AtomicBool = AtomicBool::new(false);
#[cfg(windows)]
static SHOULD_STOP: AtomicBool = AtomicBool::new(false);
#[cfg(windows)]
static OBSERVE_ALL_KEYS: AtomicBool = AtomicBool::new(false);
#[cfg(windows)]
static MULTI_JLH_MODE: AtomicBool = AtomicBool::new(false);
#[cfg(windows)]
static ACTIVE_KEYS: Mutex<ActiveKeys> = Mutex::new(ActiveKeys::new());
#[cfg(windows)]
static OBSERVED_L_EVENTS: AtomicU64 = AtomicU64::new(0);
#[cfg(windows)]
static OBSERVED_ALL_KEY_EVENTS: AtomicU64 = AtomicU64::new(0);
#[cfg(windows)]
static LAST_KEY_VK: AtomicU64 = AtomicU64::new(0);
#[cfg(windows)]
static LAST_KEY_SCAN: AtomicU64 = AtomicU64::new(0);
#[cfg(windows)]
static LAST_KEY_MESSAGE: AtomicU64 = AtomicU64::new(0);
#[cfg(windows)]
static LAST_KEY_INJECTED: AtomicBool = AtomicBool::new(false);
#[cfg(windows)]
static SUPPRESSED_PHYSICAL_DOWN: AtomicU64 = AtomicU64::new(0);
#[cfg(windows)]
static SUPPRESSED_PHYSICAL_UP: AtomicU64 = AtomicU64::new(0);
#[cfg(windows)]
static SUPPRESSED_PHYSICAL_REPEAT: AtomicU64 = AtomicU64::new(0);
#[cfg(windows)]
static OBSERVED_OWN_INJECTED: AtomicU64 = AtomicU64::new(0);
#[cfg(windows)]
static OBSERVED_OTHER_INJECTED: AtomicU64 = AtomicU64::new(0);
#[cfg(windows)]
static SENT_TAPS: AtomicU64 = AtomicU64::new(0);
#[cfg(windows)]
static SEND_FAILURES: AtomicU64 = AtomicU64::new(0);

#[cfg(windows)]
#[derive(Clone, Copy, Eq, PartialEq)]
enum FireKey {
    J,
    L,
    H,
}

#[cfg(windows)]
struct ActiveKeys {
    order: Vec<FireKey>,
    version: u64,
}

#[cfg(windows)]
impl ActiveKeys {
    const fn new() -> Self {
        Self {
            order: Vec::new(),
            version: 0,
        }
    }
}

#[cfg(windows)]
impl FireKey {
    fn from_event(vk_code: u32, scan_code: u32) -> Option<Self> {
        match (vk_code, scan_code as u16) {
            (VK_J, _) | (_, SCAN_J) => Some(Self::J),
            (VK_L, _) | (_, SCAN_L) => Some(Self::L),
            (VK_H, _) | (_, SCAN_H) => Some(Self::H),
            _ => None,
        }
    }

    fn vk(self) -> u16 {
        match self {
            Self::J => VK_J as u16,
            Self::L => VK_L as u16,
            Self::H => VK_H as u16,
        }
    }

    fn scan(self) -> u16 {
        match self {
            Self::J => SCAN_J,
            Self::L => SCAN_L,
            Self::H => SCAN_H,
        }
    }

    fn label(self) -> &'static str {
        match self {
            Self::J => "J",
            Self::L => "L",
            Self::H => "H",
        }
    }

    fn held(self) -> &'static AtomicBool {
        match self {
            Self::J => &PHYSICAL_J_HELD,
            Self::L => &PHYSICAL_L_HELD,
            Self::H => &PHYSICAL_H_HELD,
        }
    }
}

#[cfg(windows)]
#[derive(Clone, Copy)]
enum Mode {
    Normal,
    SuppressOnly,
    ObserveAll,
    TestSend,
}

#[cfg(windows)]
impl Mode {
    fn parse(value: &str) -> Result<Self, String> {
        match value {
            "normal" => Ok(Self::Normal),
            "suppress-only" => Ok(Self::SuppressOnly),
            "observe-all" => Ok(Self::ObserveAll),
            "test-send" => Ok(Self::TestSend),
            _ => Err(format!("未知 --mode 值：{value}\n{}", usage())),
        }
    }

    fn label(self) -> &'static str {
        match self {
            Self::Normal => "正常连发",
            Self::SuppressOnly => "只拦截不连发",
            Self::ObserveAll => "观察所有按键",
            Self::TestSend => "倒计时直接发送",
        }
    }
}

#[cfg(windows)]
#[derive(Clone, Copy)]
enum SendMode {
    Scancode,
    Vk,
    Mixed,
    Game,
}

#[cfg(windows)]
impl SendMode {
    fn parse(value: &str) -> Result<Self, String> {
        match value {
            "scancode" => Ok(Self::Scancode),
            "vk" => Ok(Self::Vk),
            "mixed" => Ok(Self::Mixed),
            "game" => Ok(Self::Game),
            _ => Err(format!("未知 --send-mode 值：{value}\n{}", usage())),
        }
    }

    fn label(self) -> &'static str {
        match self {
            Self::Scancode => "扫描码",
            Self::Vk => "虚拟键",
            Self::Mixed => "虚拟键+扫描码",
            Self::Game => "旧版游戏模式(vkFF+扫描码)",
        }
    }
}

#[cfg(windows)]
#[derive(Clone, Copy)]
struct Config {
    mode: Mode,
    send_mode: SendMode,
    multi_jlh: bool,
    max_speed: bool,
    rate_per_second: f64,
    down_ms: f64,
    up_ms: f64,
    test_count: u32,
}

#[cfg(windows)]
impl Config {
    fn from_args() -> Result<Self, String> {
        let mut config = Self {
            mode: Mode::Normal,
            send_mode: SendMode::Game,
            multi_jlh: false,
            max_speed: false,
            rate_per_second: 60.0,
            down_ms: 8.0,
            up_ms: 8.0,
            test_count: 20,
        };
        let mut args = env::args().skip(1);

        while let Some(arg) = args.next() {
            match arg.as_str() {
                "--mode" => {
                    let value = args.next().ok_or_else(|| "--mode 需要一个值".to_string())?;
                    config.mode = Mode::parse(&value)?;
                }
                "--send-mode" => {
                    let value = args
                        .next()
                        .ok_or_else(|| "--send-mode 需要一个值".to_string())?;
                    config.send_mode = SendMode::parse(&value)?;
                }
                "--multi-jlh" => config.multi_jlh = true,
                "--max-speed" => config.max_speed = true,
                "--rate" => config.rate_per_second = parse_value(&arg, args.next())?,
                "--down-ms" => config.down_ms = parse_value(&arg, args.next())?,
                "--up-ms" => config.up_ms = parse_value(&arg, args.next())?,
                "--test-count" => config.test_count = parse_u32_value(&arg, args.next())?,
                "--help" | "-h" => return Err(usage()),
                unknown => return Err(format!("未知参数：{unknown}\n{}", usage())),
            }
        }

        if !config.rate_per_second.is_finite() || config.rate_per_second <= 0.0 {
            return Err("--rate 必须大于 0".to_string());
        }
        if !config.down_ms.is_finite() || config.down_ms < 0.0 {
            return Err("--down-ms 必须大于或等于 0".to_string());
        }
        if !config.up_ms.is_finite() || config.up_ms < 0.0 {
            return Err("--up-ms 必须大于或等于 0".to_string());
        }
        if config.test_count == 0 {
            return Err("--test-count 必须大于 0".to_string());
        }

        Ok(config)
    }
}

#[cfg(windows)]
struct HookGuard(HHOOK);

#[cfg(windows)]
impl Drop for HookGuard {
    fn drop(&mut self) {
        unsafe {
            if !self.0.is_null() {
                UnhookWindowsHookEx(self.0);
            }
        }
    }
}

#[cfg(windows)]
fn main() {
    let config = match Config::from_args() {
        Ok(config) => config,
        Err(message) => {
            eprintln!("{message}");
            return;
        }
    };

    println!("DNF L 键 Hook 连发实验程序");
    if config.multi_jlh {
        println!("多键模式下：物理 J/L/H 会被拦截；程序按后按先发规则发送 synthetic J/L/H。");
    } else {
        println!("普通模式下：物理 L 会被拦截；程序自己发送的 synthetic L 会放行。");
    }
    println!(
        "模式={}，发送方式={}，{}，按下={}ms，抬起后等待={}ms",
        config.mode.label(),
        config.send_mode.label(),
        config.speed_label(),
        config.down_ms,
        config.up_ms
    );
    if config.multi_jlh {
        println!("请把 DNF 切到前台，然后按住物理 J/L/H。后按的键会在下一次发送周期先触发。按 Ctrl+C 退出。");
    } else {
        println!("请把 DNF 切到前台，然后按住物理 L。按 Ctrl+C 退出。");
    }
    println!("如果 DNF 是管理员权限运行，本程序也需要用管理员权限启动，否则可能完全没效果。");
    if matches!(config.mode, Mode::ObserveAll) {
        println!("观察所有按键模式不会拦截，也不会连发；请先在普通窗口按 A、L、空格测试 hook 是否能看到事件。");
        OBSERVE_ALL_KEYS.store(true, Ordering::Relaxed);
    }
    MULTI_JLH_MODE.store(config.multi_jlh, Ordering::Relaxed);
    if matches!(config.mode, Mode::TestSend) {
        println!("倒计时直接发送模式不会拦截物理 L。请在 3 秒内切到记事本或 DNF。程序会直接发送指定次数的 L。");
    }

    let hook = match install_keyboard_hook() {
        Ok(hook) => hook,
        Err(message) => {
            eprintln!("安装键盘 hook 失败：{message}");
            return;
        }
    };
    if config.multi_jlh {
        println!("键盘 hook 已安装。现在按 J/L/H 时，下面的统计应该会变化。");
    } else {
        println!("键盘 hook 已安装。现在按 L 时，下面的统计应该会变化。");
    }

    let fire_thread = thread::spawn(move || match config.mode {
        Mode::TestSend => test_send_loop(config),
        _ => fire_loop(config),
    });
    let stats_thread = thread::spawn(stats_loop);

    unsafe {
        let mut message: MSG = std::mem::zeroed();
        while GetMessageW(&mut message, null_mut(), 0, 0) > 0 {
            TranslateMessage(&message);
            DispatchMessageW(&message);
        }
    }

    SHOULD_STOP.store(true, Ordering::Relaxed);
    drop(hook);
    let _ = fire_thread.join();
    let _ = stats_thread.join();
    print_summary();
}

#[cfg(not(windows))]
fn main() {
    eprintln!("l-hook-fire-probe 只支持 Windows。");
}

#[cfg(windows)]
unsafe extern "system" fn low_level_keyboard_proc(
    code: i32,
    w_param: WPARAM,
    l_param: LPARAM,
) -> LRESULT {
    if code != HC_ACTION as i32 {
        return CallNextHookEx(null_mut(), code, w_param, l_param);
    }

    let event = &*(l_param as *const KBDLLHOOKSTRUCT);
    let injected = (event.flags & LLKHF_INJECTED) != 0;
    if OBSERVE_ALL_KEYS.load(Ordering::Relaxed) {
        OBSERVED_ALL_KEY_EVENTS.fetch_add(1, Ordering::Relaxed);
        LAST_KEY_VK.store(event.vkCode as u64, Ordering::Relaxed);
        LAST_KEY_SCAN.store(event.scanCode as u64, Ordering::Relaxed);
        LAST_KEY_MESSAGE.store(w_param as u64, Ordering::Relaxed);
        LAST_KEY_INJECTED.store(injected, Ordering::Relaxed);
        return CallNextHookEx(null_mut(), code, w_param, l_param);
    }

    let Some(key) = FireKey::from_event(event.vkCode, event.scanCode) else {
        return CallNextHookEx(null_mut(), code, w_param, l_param);
    };
    if !MULTI_JLH_MODE.load(Ordering::Relaxed) && key != FireKey::L {
        return CallNextHookEx(null_mut(), code, w_param, l_param);
    }
    OBSERVED_L_EVENTS.fetch_add(1, Ordering::Relaxed);

    if injected {
        if event.dwExtraInfo == EXTRA_INFO_MARKER {
            OBSERVED_OWN_INJECTED.fetch_add(1, Ordering::Relaxed);
        } else {
            OBSERVED_OTHER_INJECTED.fetch_add(1, Ordering::Relaxed);
        }
        return CallNextHookEx(null_mut(), code, w_param, l_param);
    }

    match w_param as u32 {
        WM_KEYDOWN | WM_SYSKEYDOWN => {
            if key.held().swap(true, Ordering::Relaxed) {
                SUPPRESSED_PHYSICAL_REPEAT.fetch_add(1, Ordering::Relaxed);
            } else {
                SUPPRESSED_PHYSICAL_DOWN.fetch_add(1, Ordering::Relaxed);
                if MULTI_JLH_MODE.load(Ordering::Relaxed) {
                    push_active_key_to_front(key);
                }
            }
            1
        }
        WM_KEYUP | WM_SYSKEYUP => {
            key.held().store(false, Ordering::Relaxed);
            SUPPRESSED_PHYSICAL_UP.fetch_add(1, Ordering::Relaxed);
            if MULTI_JLH_MODE.load(Ordering::Relaxed) {
                remove_active_key(key);
            }
            1
        }
        _ => CallNextHookEx(null_mut(), code, w_param, l_param),
    }
}

#[cfg(windows)]
fn install_keyboard_hook() -> Result<HookGuard, String> {
    let module: HINSTANCE = unsafe { GetModuleHandleW(null_mut()) };
    let hook =
        unsafe { SetWindowsHookExW(WH_KEYBOARD_LL, Some(low_level_keyboard_proc), module, 0) };

    if hook.is_null() {
        Err(format!("SetWindowsHookExW failed: {}", unsafe {
            GetLastError()
        }))
    } else {
        Ok(HookGuard(hook))
    }
}

#[cfg(windows)]
fn fire_loop(config: Config) {
    let period = Duration::from_secs_f64(1.0 / config.rate_per_second);
    let down_duration = Duration::from_secs_f64(config.down_ms / 1000.0);
    let up_duration = Duration::from_secs_f64(config.up_ms / 1000.0);
    let mut next_fire = Instant::now();
    let mut multi_cursor = 0usize;
    let mut seen_order_version = 0u64;

    while !SHOULD_STOP.load(Ordering::Relaxed) {
        if any_trigger_held(config.multi_jlh) {
            if matches!(config.mode, Mode::SuppressOnly | Mode::ObserveAll) {
                thread::sleep(Duration::from_millis(1));
                continue;
            }

            let now = Instant::now();
            if config.max_speed || now >= next_fire {
                let key = if config.multi_jlh {
                    next_multi_key(&mut multi_cursor, &mut seen_order_version)
                } else {
                    Some(FireKey::L)
                };

                let send_result = if let Some(key) = key {
                    send_key_tap(key, config.send_mode, down_duration, up_duration)
                } else {
                    thread::sleep(Duration::from_millis(1));
                    continue;
                };

                match send_result {
                    Ok(()) => {
                        SENT_TAPS.fetch_add(1, Ordering::Relaxed);
                    }
                    Err(error) => {
                        SEND_FAILURES.fetch_add(1, Ordering::Relaxed);
                        eprintln!("SendInput 失败：{error}");
                    }
                }
                next_fire = if config.max_speed {
                    Instant::now()
                } else {
                    now + period
                };
            }
            if !config.max_speed {
                precise_sleep_until(next_fire);
            }
        } else {
            next_fire = Instant::now();
            thread::sleep(Duration::from_millis(1));
        }
    }
}

#[cfg(windows)]
fn test_send_loop(config: Config) {
    for remaining in (1..=3).rev() {
        println!("{} 秒后开始直接发送 L，请切到目标窗口...", remaining);
        thread::sleep(Duration::from_secs(1));
    }

    let down_duration = Duration::from_secs_f64(config.down_ms / 1000.0);
    let up_duration = Duration::from_secs_f64(config.up_ms / 1000.0);
    for index in 1..=config.test_count {
        match send_key_tap(FireKey::L, config.send_mode, down_duration, up_duration) {
            Ok(()) => {
                SENT_TAPS.fetch_add(1, Ordering::Relaxed);
                println!("已直接发送 L：{}/{}", index, config.test_count);
            }
            Err(error) => {
                SEND_FAILURES.fetch_add(1, Ordering::Relaxed);
                eprintln!("直接发送 L 失败：{error}");
            }
        }
    }

    println!("直接发送测试完成。请按 Ctrl+C 退出。当前统计如下：");
    print_summary();
}

#[cfg(windows)]
fn send_key_tap(
    key: FireKey,
    send_mode: SendMode,
    down_duration: Duration,
    up_duration: Duration,
) -> Result<(), String> {
    send_key_event(key, send_mode, false)?;
    precise_sleep(down_duration);
    send_key_event(key, send_mode, true)?;
    precise_sleep(up_duration);

    Ok(())
}

#[cfg(windows)]
fn send_key_event(key: FireKey, send_mode: SendMode, key_up: bool) -> Result<(), String> {
    let key_up_flag = if key_up { KEYEVENTF_KEYUP } else { 0 };
    let flags = match send_mode {
        SendMode::Scancode => key_up_flag | KEYEVENTF_SCANCODE,
        SendMode::Vk => key_up_flag,
        SendMode::Mixed => key_up_flag | KEYEVENTF_SCANCODE,
        SendMode::Game => key_up_flag,
    };
    let vk = match send_mode {
        SendMode::Scancode => 0,
        SendMode::Vk | SendMode::Mixed => key.vk(),
        SendMode::Game => 0xFF,
    };
    let scan = match send_mode {
        SendMode::Vk => 0,
        SendMode::Scancode | SendMode::Mixed | SendMode::Game => key.scan(),
    };
    let input = INPUT {
        r#type: INPUT_KEYBOARD,
        Anonymous: INPUT_0 {
            ki: KEYBDINPUT {
                wVk: vk,
                wScan: scan,
                dwFlags: flags,
                time: 0,
                dwExtraInfo: EXTRA_INFO_MARKER,
            },
        },
    };

    let sent = unsafe { SendInput(1, &input, size_of::<INPUT>() as i32) };
    if sent == 1 {
        Ok(())
    } else {
        Err(format!(
            "SendInput returned {sent}; GetLastError={}",
            unsafe { GetLastError() }
        ))
    }
}

#[cfg(windows)]
fn precise_sleep(duration: Duration) {
    if duration.is_zero() {
        return;
    }

    precise_sleep_until(Instant::now() + duration);
}

#[cfg(windows)]
fn precise_sleep_until(deadline: Instant) {
    loop {
        let now = Instant::now();
        if now >= deadline {
            break;
        }

        let remaining = deadline - now;
        if remaining > Duration::from_millis(2) {
            thread::sleep(remaining - Duration::from_millis(1));
        } else {
            std::hint::spin_loop();
        }
    }
}

#[cfg(windows)]
fn stats_loop() {
    while !SHOULD_STOP.load(Ordering::Relaxed) {
        thread::sleep(Duration::from_secs(1));
        print_summary();
    }
}

#[cfg(windows)]
fn print_summary() {
    println!(
        "状态：前台窗口=\"{}\" | 看到全部键盘事件={} | 最近键={} | 看到目标键事件={} | 已发送连发={} | 看到自己注入事件={} | 拦截物理按下={} | 拦截自动重复={} | 拦截物理抬起={} | 其他注入={} | 发送失败={} | 当前按住={} | 多键队列={}",
        foreground_window_title(),
        OBSERVED_ALL_KEY_EVENTS.load(Ordering::Relaxed),
        last_key_summary(),
        OBSERVED_L_EVENTS.load(Ordering::Relaxed),
        SENT_TAPS.load(Ordering::Relaxed),
        OBSERVED_OWN_INJECTED.load(Ordering::Relaxed),
        SUPPRESSED_PHYSICAL_DOWN.load(Ordering::Relaxed),
        SUPPRESSED_PHYSICAL_REPEAT.load(Ordering::Relaxed),
        SUPPRESSED_PHYSICAL_UP.load(Ordering::Relaxed),
        OBSERVED_OTHER_INJECTED.load(Ordering::Relaxed),
        SEND_FAILURES.load(Ordering::Relaxed),
        held_summary(),
        active_order_summary(),
    );
}

#[cfg(windows)]
fn parse_value(flag: &str, value: Option<String>) -> Result<f64, String> {
    value
        .ok_or_else(|| format!("{flag} 需要一个值"))?
        .parse::<f64>()
        .map_err(|error| format!("{flag} 的值无效：{error}"))
}

#[cfg(windows)]
fn usage() -> String {
    "用法：l-hook-fire-probe [--mode normal|suppress-only|observe-all|test-send] [--send-mode scancode|vk|mixed|game] [--multi-jlh] [--max-speed] [--rate 60] [--down-ms 8] [--up-ms 8] [--test-count 20]"
        .to_string()
}

#[cfg(windows)]
impl Config {
    fn speed_label(self) -> String {
        if self.max_speed {
            "极速模式：不按 rate 节流".to_string()
        } else {
            format!("目标频率={} 次/秒", self.rate_per_second)
        }
    }
}

#[cfg(windows)]
fn parse_u32_value(flag: &str, value: Option<String>) -> Result<u32, String> {
    value
        .ok_or_else(|| format!("{flag} 需要一个值"))?
        .parse::<u32>()
        .map_err(|error| format!("{flag} 的值无效：{error}"))
}

#[cfg(windows)]
fn last_key_summary() -> String {
    let vk = LAST_KEY_VK.load(Ordering::Relaxed);
    let scan = LAST_KEY_SCAN.load(Ordering::Relaxed);
    let message = LAST_KEY_MESSAGE.load(Ordering::Relaxed);
    let injected = LAST_KEY_INJECTED.load(Ordering::Relaxed);
    if vk == 0 && scan == 0 && message == 0 {
        return "无".to_string();
    }

    format!("vk=0x{vk:02X}, scan=0x{scan:02X}, msg=0x{message:04X}, injected={injected}")
}

#[cfg(windows)]
fn foreground_window_title() -> String {
    let window: HWND = unsafe { GetForegroundWindow() };
    if window.is_null() {
        return "无前台窗口".to_string();
    }

    let mut buffer = [0u16; 256];
    let length = unsafe { GetWindowTextW(window, buffer.as_mut_ptr(), buffer.len() as i32) };
    if length <= 0 {
        return "无法读取标题".to_string();
    }

    String::from_utf16_lossy(&buffer[..length as usize])
}

#[cfg(windows)]
fn push_active_key_to_front(key: FireKey) {
    let Ok(mut active_keys) = ACTIVE_KEYS.lock() else {
        return;
    };
    active_keys.order.retain(|existing| *existing != key);
    active_keys.order.insert(0, key);
    active_keys.version = active_keys.version.wrapping_add(1);
}

#[cfg(windows)]
fn remove_active_key(key: FireKey) {
    let Ok(mut active_keys) = ACTIVE_KEYS.lock() else {
        return;
    };
    let original_len = active_keys.order.len();
    active_keys.order.retain(|existing| *existing != key);
    if active_keys.order.len() != original_len {
        active_keys.version = active_keys.version.wrapping_add(1);
    }
}

#[cfg(windows)]
fn any_trigger_held(multi_jlh: bool) -> bool {
    if multi_jlh {
        PHYSICAL_J_HELD.load(Ordering::Relaxed)
            || PHYSICAL_L_HELD.load(Ordering::Relaxed)
            || PHYSICAL_H_HELD.load(Ordering::Relaxed)
    } else {
        PHYSICAL_L_HELD.load(Ordering::Relaxed)
    }
}

#[cfg(windows)]
fn next_multi_key(cursor: &mut usize, seen_version: &mut u64) -> Option<FireKey> {
    let Ok(active_keys) = ACTIVE_KEYS.lock() else {
        return None;
    };
    if active_keys.order.is_empty() {
        return None;
    }
    if active_keys.version != *seen_version || *cursor >= active_keys.order.len() {
        *cursor = 0;
        *seen_version = active_keys.version;
    }
    let key = active_keys.order[*cursor];
    *cursor = (*cursor + 1) % active_keys.order.len();
    Some(key)
}

#[cfg(windows)]
fn held_summary() -> String {
    [FireKey::J, FireKey::L, FireKey::H]
        .into_iter()
        .filter(|key| key.held().load(Ordering::Relaxed))
        .map(FireKey::label)
        .collect::<Vec<_>>()
        .join("")
}

#[cfg(windows)]
fn active_order_summary() -> String {
    let Ok(active_keys) = ACTIVE_KEYS.lock() else {
        return "锁定失败".to_string();
    };
    if active_keys.order.is_empty() {
        return "空".to_string();
    }
    active_keys
        .order
        .iter()
        .map(|key| key.label())
        .collect::<Vec<_>>()
        .join("")
}
