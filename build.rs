//! 构建脚本 - 嵌入 Windows 应用程序清单，请求管理员权限

fn main() {
    // 只在 Windows 上执行
    #[cfg(target_os = "windows")]
    {
        let mut res = winres::WindowsResource::new();

        // 请求管理员权限
        res.set_manifest(r#"
<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
<assembly xmlns="urn:schemas-microsoft-com:asm.v1" manifestVersion="1.0">
  <trustInfo xmlns="urn:schemas-microsoft-com:asm.v3">
    <security>
      <requestedPrivileges>
        <requestedExecutionLevel level="requireAdministrator" uiAccess="false"/>
      </requestedPrivileges>
    </security>
  </trustInfo>
</assembly>
"#);

        res.compile().expect("Failed to compile Windows resource");
    }
}
