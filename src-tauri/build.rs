fn main() {
    // 嵌入 Windows 清单文件，请求管理员权限
    #[cfg(windows)]
    {
        let mut res = tauri_build::WindowsAttributes::new();
        res = res.app_manifest(include_str!("app.manifest"));
        tauri_build::try_build(tauri_build::Attributes::new().windows_attributes(res))
            .expect("无法构建 Tauri 应用");
    }

    #[cfg(not(windows))]
    tauri_build::build();
}
