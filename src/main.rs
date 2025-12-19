mod core;

use core::{vk, AutoFireEngine};
use std::io::{self, Write};

#[cfg(windows)]
fn is_elevated() -> bool {
    use windows::Win32::Foundation::HANDLE;
    use windows::Win32::Security::{GetTokenInformation, TokenElevation, TOKEN_ELEVATION, TOKEN_QUERY};
    use windows::Win32::System::Threading::{GetCurrentProcess, OpenProcessToken};

    unsafe {
        let mut token = HANDLE::default();
        if OpenProcessToken(GetCurrentProcess(), TOKEN_QUERY, &mut token).is_err() {
            return false;
        }

        let mut elevation = TOKEN_ELEVATION { TokenIsElevated: 0 };
        let mut size = 0u32;
        let result = GetTokenInformation(
            token,
            TokenElevation,
            Some(&mut elevation as *mut _ as *mut _),
            std::mem::size_of::<TOKEN_ELEVATION>() as u32,
            &mut size,
        );

        result.is_ok() && elevation.TokenIsElevated != 0
    }
}

fn main() {
    println!("=======================================");
    println!("  DNF AutoFire v0.1.0 - 连发测试程序  ");
    println!("=======================================\n");

    // 检查管理员权限（清单文件应该自动提示 UAC）
    #[cfg(windows)]
    {
        if !is_elevated() {
            eprintln!("❌ 错误：程序未以管理员权限运行！");
            eprintln!("\n这不应该发生，因为程序应该自动请求管理员权限。");
            eprintln!("\n如果你看到这条消息，说明：");
            eprintln!("1. UAC 提示被拒绝了");
            eprintln!("2. 或者清单文件嵌入失败\n");
            eprintln!("按 Enter 退出...");
            let mut input = String::new();
            io::stdin().read_line(&mut input).unwrap();
            std::process::exit(1);
        }
        println!("✓ 管理员权限验证通过\n");
    }

    println!("功能说明：");
    println!("- 当你在记事本中按住 J 或 L 键时，程序会自动连发");
    println!("- 只在记事本窗口激活时工作");
    println!("- 按键间隔：约 33ms（每秒 30 次）\n");

    // 创建连发引擎
    let mut engine = AutoFireEngine::new();

    // 配置连发按键：J 和 L
    engine.set_keys(vec![vk::VK_J, vk::VK_L]);

    println!("已配置连发按键：J, L\n");
    println!("请先打开记事本，然后按 Enter 启动连发引擎...");

    // 等待用户按 Enter
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    // 启动连发引擎
    engine.start();

    println!("\n连发引擎已启动！");
    println!("==============================");
    println!("现在你可以：");
    println!("1. 切换到记事本窗口");
    println!("2. 按住 J 或 L 键测试连发效果");
    println!("3. 按 Enter 停止程序\n");
    println!("提示：");
    println!("- 按住 J 键 → 持续输入 'jjjjj...'");
    println!("- 按住 L 键 → 持续输入 'lllll...'");
    println!("==============================\n");

    // 等待用户按 Enter 停止
    print!("按 Enter 停止连发引擎...");
    io::stdout().flush().unwrap();
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    // 停止连发引擎
    engine.stop();

    println!("\n连发引擎已停止。");
    println!("感谢使用！");
}
