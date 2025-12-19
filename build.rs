fn main() {
    // 仅在 Windows 平台编译资源
    #[cfg(windows)]
    {
        // 编译资源文件 (.rc) 并嵌入到可执行文件
        embed_resource::compile("app.rc", embed_resource::NONE);
    }
}
