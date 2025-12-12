//! DNF AutoFire Tool - 高性能连发工具
//!
//! 使用 Rust + egui 重写，比 AHK 版本更快更稳定

// #![windows_subsystem = "windows"] // 调试时注释掉，发布时取消注释

mod core;
mod config;
mod gui;

use eframe::egui;

fn main() -> eframe::Result<()> {
    // 设置日志（调试时可取消注释）
    // env_logger::init();

    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_title("DNF AutoFire Tool v0.1.0")
            .with_inner_size([600.0, 500.0])
            .with_min_inner_size([500.0, 400.0])
            .with_resizable(true),
        ..Default::default()
    };

    eframe::run_native(
        "DNF AutoFire",
        options,
        Box::new(|cc| Ok(Box::new(gui::App::new(cc)))),
    )
}
