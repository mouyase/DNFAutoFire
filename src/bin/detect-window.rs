//! 窗口类名检测工具
//!
//! 用于检测当前活动窗口的类名，帮助配置连发程序

use std::thread;
use std::time::Duration;
use windows::Win32::Foundation::HWND;
use windows::Win32::UI::WindowsAndMessaging::{GetClassNameW, GetForegroundWindow};

fn get_foreground_class_name() -> String {
    unsafe {
        let hwnd = GetForegroundWindow();
        if hwnd == HWND::default() {
            return String::new();
        }

        let mut class_name_buf = [0u16; 256];
        let len = GetClassNameW(hwnd, &mut class_name_buf);
        if len == 0 {
            return String::new();
        }

        String::from_utf16_lossy(&class_name_buf[..len as usize])
    }
}

fn main() {
    println!("========================================");
    println!("   DNF AutoFire - 窗口类名检测工具");
    println!("========================================\n");

    println!("功能：每 2 秒检测一次当前活动窗口的类名");
    println!("用途：找到 DNF 游戏窗口的实际类名\n");
    println!("使用方法：");
    println!("1. 运行此程序");
    println!("2. 切换到 DNF 游戏窗口");
    println!("3. 观察输出的窗口类名");
    println!("4. 将类名添加到连发程序配置中\n");
    println!("按 Ctrl+C 停止检测\n");
    println!("----------------------------------------\n");

    let mut last_class_name = String::new();

    loop {
        let class_name = get_foreground_class_name();

        if !class_name.is_empty() && class_name != last_class_name {
            println!("当前窗口类名: \"{}\"", class_name);
            last_class_name = class_name;
        }

        thread::sleep(Duration::from_secs(2));
    }
}
