//! 系统托盘模块
//!
//! 负责创建和管理系统托盘图标及菜单

use tauri::{
    menu::{Menu, MenuItem},
    tray::{MouseButton, MouseButtonState, TrayIconBuilder, TrayIconEvent},
    AppHandle, Emitter, Manager, Runtime,
};

/// 托盘菜单项 ID
pub mod menu_ids {
    pub const TOGGLE_AUTOFIRE: &str = "toggle_autofire";
    pub const SHOW_MAIN: &str = "show_main";
    pub const QUIT: &str = "quit";
}

/// 创建系统托盘
pub fn create_tray<R: Runtime>(app: &AppHandle<R>) -> tauri::Result<()> {
    // 创建菜单
    let menu = create_menu(app, false)?;

    // 创建托盘图标
    let _tray = TrayIconBuilder::with_id("main-tray")
        .icon(app.default_window_icon().unwrap().clone())
        .menu(&menu)
        .show_menu_on_left_click(false) // 左键不显示菜单
        .tooltip("DNF 连发工具")
        .on_menu_event(handle_menu_event)
        .on_tray_icon_event(handle_tray_event)
        .build(app)?;

    println!("[托盘] 系统托盘已创建");
    Ok(())
}

/// 创建托盘菜单
fn create_menu<R: Runtime>(app: &AppHandle<R>, is_running: bool) -> tauri::Result<Menu<R>> {
    let autofire_text = if is_running { "停止连发" } else { "启动连发" };

    let toggle_autofire = MenuItem::with_id(
        app,
        menu_ids::TOGGLE_AUTOFIRE,
        autofire_text,
        true,
        None::<&str>,
    )?;

    let show_main = MenuItem::with_id(app, menu_ids::SHOW_MAIN, "显示主窗口", true, None::<&str>)?;

    let quit = MenuItem::with_id(app, menu_ids::QUIT, "退出", true, None::<&str>)?;

    Menu::with_items(app, &[&toggle_autofire, &show_main, &quit])
}

/// 处理菜单事件
fn handle_menu_event<R: Runtime>(app: &AppHandle<R>, event: tauri::menu::MenuEvent) {
    match event.id.as_ref() {
        menu_ids::TOGGLE_AUTOFIRE => {
            // 通过事件通知前端切换连发
            if let Some(window) = app.get_webview_window("main") {
                let _ = window.emit("tray-toggle-autofire", ());
                println!("[托盘] 发送连发切换事件");
            }
        }
        menu_ids::SHOW_MAIN => {
            show_main_window(app);
        }
        menu_ids::QUIT => {
            println!("[托盘] 退出应用");
            app.exit(0);
        }
        _ => {}
    }
}

/// 处理托盘图标点击事件（左键点击显示主窗口）
fn handle_tray_event<R: Runtime>(tray: &tauri::tray::TrayIcon<R>, event: TrayIconEvent) {
    if let TrayIconEvent::Click {
        button: MouseButton::Left,
        button_state: MouseButtonState::Up,
        ..
    } = event
    {
        show_main_window(tray.app_handle());
    }
}

/// 显示主窗口
pub fn show_main_window<R: Runtime>(app: &AppHandle<R>) {
    if let Some(window) = app.get_webview_window("main") {
        let _ = window.show();
        let _ = window.set_focus();
        println!("[托盘] 显示主窗口");
    }
}

/// 隐藏主窗口
pub fn hide_main_window<R: Runtime>(app: &AppHandle<R>) {
    if let Some(window) = app.get_webview_window("main") {
        let _ = window.hide();
        println!("[托盘] 隐藏主窗口");
    }
}

/// 切换迷你窗口显示状态
pub fn toggle_mini_window<R: Runtime>(app: &AppHandle<R>) {
    if let Some(window) = app.get_webview_window("mini") {
        if window.is_visible().unwrap_or(false) {
            let _ = window.hide();
            println!("[托盘] 隐藏迷你窗口");
        } else {
            let _ = window.show();
            let _ = window.set_focus();
            let _ = window.set_always_on_top(true);
            println!("[托盘] 显示迷你窗口");
        }
    }
}

/// 更新托盘菜单中的连发状态文本
pub fn update_autofire_menu<R: Runtime>(app: &AppHandle<R>, is_running: bool) {
    // 重新创建菜单并设置到托盘
    if let Some(tray) = app.tray_by_id("main-tray") {
        if let Ok(menu) = create_menu(app, is_running) {
            let _ = tray.set_menu(Some(menu));
            let text = if is_running { "停止连发" } else { "启动连发" };
            println!("[托盘] 更新菜单状态: {}", text);
        }
    }
}
