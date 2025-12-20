mod core;
mod gui;

use gui::{App, AppState};
use native_windows_gui as nwg;
use std::cell::RefCell;
use std::io;
use std::rc::Rc;

#[cfg(windows)]
fn is_elevated() -> bool {
    use windows::Win32::Foundation::HANDLE;
    use windows::Win32::Security::{
        GetTokenInformation, TokenElevation, TOKEN_ELEVATION, TOKEN_QUERY,
    };
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
    }

    // 初始化 NWG
    nwg::init().expect("无法初始化 Native Windows GUI");

    // 设置字体
    nwg::Font::set_global_family("Microsoft YaHei UI").expect("无法设置字体");

    // 创建应用程序状态
    let state = Rc::new(RefCell::new(AppState::new()));

    // 构建 UI
    let app = App::build_ui(state.clone()).expect("无法构建 UI");

    // 初始化数据
    app.init();

    // 运行消息循环
    nwg::dispatch_thread_events();
}
