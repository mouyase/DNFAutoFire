# DNF AutoFire

高性能 DNF 游戏连发工具，使用 Rust 编写。

## 特性

- 高性能按键连发
- 支持多键同时连发
- 聊天框兼容（聊天时不会连发）
- 低延迟、低资源占用
- 简洁的 GUI 界面

## 技术原理

详见 [TECHNICAL_NOTES.md](./TECHNICAL_NOTES.md)

## 环境要求

- Windows 10/11
- Rust 1.70+（GNU 工具链）
- 管理员权限运行

## 安装 Rust

```bash
# 下载 rustup-init.exe 并运行
# 选择 GNU 工具链（不需要 Visual Studio）
rustup default stable-x86_64-pc-windows-gnu
```

## 编译

```bash
# 开发版本
cargo build

# 发布版本（优化）
cargo build --release
```

## 运行

```bash
# 以管理员身份运行
cargo run --release
```

或直接运行编译后的 exe：
```
target\release\dnf-autofire.exe
```

## 开发

```bash
# 运行测试程序
cargo run --bin test_multi2

# 代码格式化
cargo fmt

# 代码检查
cargo clippy
```

## 项目结构

```
dnf-autofire/
├── src/
│   ├── main.rs           # 主程序入口
│   ├── core/             # 核心逻辑
│   │   ├── mod.rs
│   │   ├── autofire.rs   # 连发引擎
│   │   ├── keyboard.rs   # 键盘输入
│   │   └── window.rs     # 窗口检测
│   ├── config/           # 配置管理
│   │   └── mod.rs
│   ├── gui/              # GUI 界面
│   │   ├── mod.rs
│   │   └── keyboard_layout.rs
│   └── bin/              # 测试程序
│       ├── test_final.rs
│       └── test_multi2.rs
├── Cargo.toml
├── build.rs              # 构建脚本（管理员清单）
├── TECHNICAL_NOTES.md    # 技术笔记
└── README.md
```

## 许可证

MIT License
