//! DNF AutoFire Tauri 后端

mod core;
mod engine;
mod hotkey;
mod tray;

use parking_lot::Mutex;
use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use std::sync::Arc;
use tauri::{Manager, State, WindowEvent};

use core::autofire::AutoFireEngine;

/// 应用状态
pub struct AppState {
    engine: Arc<Mutex<AutoFireEngine>>,
    enabled_keys: Arc<Mutex<HashSet<u16>>>,
}

impl Default for AppState {
    fn default() -> Self {
        Self {
            engine: Arc::new(Mutex::new(AutoFireEngine::new())),
            enabled_keys: Arc::new(Mutex::new(HashSet::new())),
        }
    }
}

/// 按键信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KeyState {
    pub vk: u16,
    pub enabled: bool,
}

/// 获取所有按键状态
#[tauri::command]
fn get_enabled_keys(state: State<AppState>) -> Vec<u16> {
    let keys = state.enabled_keys.lock();
    keys.iter().copied().collect()
}

/// 切换按键状态
#[tauri::command]
fn toggle_key(vk: u16, state: State<AppState>) -> bool {
    let mut keys = state.enabled_keys.lock();
    let enabled = if keys.contains(&vk) {
        keys.remove(&vk);
        false
    } else {
        keys.insert(vk);
        true
    };

    // 同步到引擎
    let engine = state.engine.lock();
    engine.set_keys(keys.iter().copied().collect());

    println!(
        "[Tauri] 按键 {:#X} 状态: {}",
        vk,
        if enabled { "启用" } else { "禁用" }
    );
    enabled
}

/// 设置所有按键
#[tauri::command]
fn set_keys(keys: Vec<u16>, state: State<AppState>) {
    let mut enabled_keys = state.enabled_keys.lock();
    enabled_keys.clear();
    for vk in &keys {
        enabled_keys.insert(*vk);
    }

    let engine = state.engine.lock();
    engine.set_keys(keys);
    println!("[Tauri] 设置按键完成");
}

/// 清空所有按键
#[tauri::command]
fn clear_keys(state: State<AppState>) {
    let mut keys = state.enabled_keys.lock();
    keys.clear();

    let engine = state.engine.lock();
    engine.set_keys(vec![]);
    println!("[Tauri] 已清空所有按键");
}

/// 启动连发
#[tauri::command]
fn start_autofire(state: State<AppState>) -> bool {
    let mut engine = state.engine.lock();
    if !engine.is_running() {
        engine.start();
        println!("[Tauri] 连发已启动");
        true
    } else {
        false
    }
}

/// 停止连发
#[tauri::command]
fn stop_autofire(state: State<AppState>) -> bool {
    let mut engine = state.engine.lock();
    if engine.is_running() {
        engine.stop();
        println!("[Tauri] 连发已停止");
        true
    } else {
        false
    }
}

/// 获取连发状态
#[tauri::command]
fn is_running(state: State<AppState>) -> bool {
    let engine = state.engine.lock();
    engine.is_running()
}

/// 获取平台信息（用于前端判断是否为 mock 模式）
#[tauri::command]
fn get_platform_info() -> PlatformInfo {
    PlatformInfo {
        platform: if cfg!(windows) {
            "windows".to_string()
        } else if cfg!(target_os = "macos") {
            "macos".to_string()
        } else if cfg!(target_os = "linux") {
            "linux".to_string()
        } else {
            "unknown".to_string()
        },
        is_mock_mode: !cfg!(windows),
    }
}

/// 平台信息
#[derive(Debug, Clone, Serialize)]
pub struct PlatformInfo {
    /// 当前平台（windows/macos/linux）
    pub platform: String,
    /// 是否为 mock 模式（非 Windows 平台）
    pub is_mock_mode: bool,
}

/// 检查是否以管理员权限运行
#[tauri::command]
fn is_elevated() -> bool {
    #[cfg(windows)]
    {
        use windows::Win32::Foundation::HANDLE;
        use windows::Win32::Security::{
            GetTokenInformation, TokenElevation, TOKEN_ELEVATION, TOKEN_QUERY,
        };
        use windows::Win32::System::Threading::{GetCurrentProcess, OpenProcessToken};

        unsafe {
            let mut token_handle = HANDLE::default();
            if OpenProcessToken(GetCurrentProcess(), TOKEN_QUERY, &mut token_handle).is_err() {
                return false;
            }

            let mut elevation = TOKEN_ELEVATION::default();
            let mut size = std::mem::size_of::<TOKEN_ELEVATION>() as u32;

            let result = GetTokenInformation(
                token_handle,
                TokenElevation,
                Some(&mut elevation as *mut _ as *mut _),
                size,
                &mut size,
            );

            let _ = windows::Win32::Foundation::CloseHandle(token_handle);

            result.is_ok() && elevation.TokenIsElevated != 0
        }
    }

    #[cfg(not(windows))]
    {
        false
    }
}

/// 以管理员权限重启
#[tauri::command]
fn restart_as_admin() -> bool {
    #[cfg(windows)]
    {
        use std::os::windows::ffi::OsStrExt;
        use windows::core::PCWSTR;
        use windows::Win32::UI::Shell::ShellExecuteW;
        use windows::Win32::UI::WindowsAndMessaging::SW_SHOWNORMAL;

        let exe_path = std::env::current_exe().unwrap_or_default();
        let exe_str: Vec<u16> = exe_path.as_os_str().encode_wide().chain(Some(0)).collect();
        let verb: Vec<u16> = "runas\0".encode_utf16().collect();

        unsafe {
            let result = ShellExecuteW(
                None,
                PCWSTR(verb.as_ptr()),
                PCWSTR(exe_str.as_ptr()),
                PCWSTR::null(),
                PCWSTR::null(),
                SW_SHOWNORMAL,
            );

            if result.0 as usize > 32 {
                std::process::exit(0);
            }
            false
        }
    }

    #[cfg(not(windows))]
    {
        false
    }
}

/// 更新托盘菜单中的连发状态
#[tauri::command]
fn update_tray_status(is_running: bool, app_handle: tauri::AppHandle) {
    tray::update_autofire_menu(&app_handle, is_running);
}

/// 隐藏主窗口到托盘
#[tauri::command]
fn hide_to_tray(app_handle: tauri::AppHandle) {
    tray::hide_main_window(&app_handle);
}

/// 显示主窗口
#[tauri::command]
fn show_main_window(app_handle: tauri::AppHandle) {
    tray::show_main_window(&app_handle);
}

/// 切换迷你窗口显示状态
#[tauri::command]
fn toggle_mini_window(app_handle: tauri::AppHandle) {
    tray::toggle_mini_window(&app_handle);
}

/// 更新快捷键配置
#[tauri::command]
fn update_shortcuts(popup_shortcut: String, toggle_shortcut: String) {
    hotkey::update_hotkey_config(&popup_shortcut, &toggle_shortcut);
}

/// 播放系统音效
#[tauri::command]
fn play_sound(sound_type: String) {
    #[cfg(windows)]
    {
        use windows::core::PCWSTR;
        use windows::Win32::Media::Audio::{PlaySoundW, SND_ALIAS, SND_ASYNC};

        let sound_name = match sound_type.as_str() {
            "start" => "SystemAsterisk\0",   // 星号提示音
            "stop" => "SystemExclamation\0", // 感叹号提示音
            _ => return,
        };

        let wide: Vec<u16> = sound_name.encode_utf16().collect();

        unsafe {
            let _ = PlaySoundW(PCWSTR(wide.as_ptr()), None, SND_ALIAS | SND_ASYNC);
        }

        println!("[音效] 播放: {}", sound_type);
    }
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_opener::init())
        .setup(|app| {
            // 创建系统托盘
            tray::create_tray(app.handle())?;

            // 启动快捷键钩子（使用键盘钩子，不拦截按键）
            #[cfg(desktop)]
            {
                hotkey::start_hotkey_hook(app.handle());
            }

            // 拦截主窗口关闭事件，改为隐藏到托盘
            if let Some(main_window) = app.get_webview_window("main") {
                let app_handle = app.handle().clone();
                main_window.on_window_event(move |event| {
                    if let WindowEvent::CloseRequested { api, .. } = event {
                        api.prevent_close();
                        tray::hide_main_window(&app_handle);
                    }
                });
            }

            // 拦截迷你窗口失焦事件，自动隐藏
            if let Some(mini_window) = app.get_webview_window("mini") {
                mini_window.on_window_event(move |event| {
                    if let WindowEvent::Focused(false) = event {
                        // 暂时注释掉自动隐藏，让用户可以用 Esc 关闭
                        // let _ = mini_window.hide();
                    }
                });
            }

            println!("[Tauri] 应用初始化完成");
            Ok(())
        })
        .manage(AppState::default())
        .invoke_handler(tauri::generate_handler![
            get_enabled_keys,
            toggle_key,
            set_keys,
            clear_keys,
            start_autofire,
            stop_autofire,
            is_running,
            get_platform_info,
            is_elevated,
            restart_as_admin,
            update_tray_status,
            hide_to_tray,
            show_main_window,
            toggle_mini_window,
            update_shortcuts,
            play_sound,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
