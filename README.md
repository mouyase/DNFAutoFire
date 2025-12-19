# DNF AutoFire

DNF 按键连发工具 - Rust 原生实现

## 项目状态

✅ **核心模块已完成**

当前版本实现了最核心的按键发送功能，作为后续开发的基础。

## 已实现功能

### 1. 核心按键发送模块 (`src/core/`)

- **KeyboardDriver Trait** - 平台无关的键盘驱动接口
  - `send_key_down()` - 发送按键按下
  - `send_key_up()` - 发送按键释放
  - `send_key_press()` - 发送完整按键（按下+释放）
  - `send_game_key_down()` - 游戏专用按键（vk=0xFF，仅游戏识别）
  - `send_game_key_up()` - 游戏专用按键释放
  - `is_key_pressed()` - 检测物理按键状态
  - `vk_to_scan_code()` - 虚拟键码转扫描码

- **WindowsKeyboardDriver** - Windows SendInput API 实现
  - 使用 Windows `SendInput` API 发送按键
  - 支持扫描码模式（`KEYEVENTF_SCANCODE`）
  - 游戏和聊天框双模式支持

- **VK 按键常量** - DNF 常用按键定义
  - 字母键：Q, W, E, R, A, S, D, F, G, H, J, K, X, Y, Z
  - 数字键：1-9（技能快捷键）
  - 功能键：Space, Shift, Control, Alt

### 2. 管理员权限

✅ 程序已配置为**自动请求管理员权限**
- 使用 Windows manifest 嵌入
- 启动时会自动弹出 UAC 提示
- 无需手动"以管理员身份运行"

## 技术栈

- **Rust** - 核心语言
- **windows-rs 0.59** - Windows API 绑定
- **winres 0.1** - Windows 资源编译器（官方推荐）

## 开发工作流

### 使用 Claude Code Skills（推荐）

本项目已配置 Claude Code skills，使用最简单：

```
/format   # 格式化代码
/lint     # 代码质量检查
/fix      # 自动修复问题
/check    # 完整检查（提交前）
```

**推荐流程**：
```
编辑代码 → /format → /lint → 继续开发
```

**提交前**：
```
/check
```

详见 `.claude/skills/README.md`

### 手动命令

**格式化代码**（每次编辑后必须运行）：
```bash
cargo fmt --all
```

**检查代码质量**（Clippy，零警告模式）：
```bash
cargo clippy --all-targets --all-features -- -D warnings
```

**完整检查**（格式化 + Clippy + 测试 + 编译）：
```powershell
.\scripts\check.ps1
```

**快速格式化**：
```powershell
.\scripts\format.ps1
```

### 使用 cargo-make（可选）

安装 cargo-make：
```bash
cargo install cargo-make
```

常用命令：
```bash
cargo make format        # 格式化代码
cargo make clippy        # 运行 Clippy
cargo make check         # 完整检查
cargo make fix           # 自动修复问题
cargo make pre-commit    # 提交前检查
```

## 编译

### Debug 版本
```bash
cargo build
```

### Release 版本（推荐）
```bash
cargo build --release
```

编译输出：
- Debug: `target/debug/dnf-autofire.exe`
- Release: `target/release/dnf-autofire.exe` (~258KB)

## 运行

双击运行 `dnf-autofire.exe`，会自动请求管理员权限。

当前版本是测试程序，会：
1. 倒计时 3 秒
2. 发送一次 Q 键
3. 退出

可以在记事本中测试按键发送是否正常工作。

## 项目结构

```
DNFAutoFire/
├── .claude/                 # Claude Code 配置
│   └── skills/              # 自定义 skills
│       ├── format/          # /format - 格式化代码
│       │   └── SKILL.md
│       ├── lint/            # /lint - 代码检查
│       │   └── SKILL.md
│       ├── fix/             # /fix - 自动修复
│       │   └── SKILL.md
│       ├── check/           # /check - 完整检查
│       │   └── SKILL.md
│       └── README.md        # Skills 说明
├── src/
│   ├── main.rs              # 主程序入口
│   └── core/                # 核心模块
│       ├── mod.rs           # 模块导出
│       └── keyboard.rs      # 按键发送实现
├── scripts/                 # 开发脚本
│   ├── check.ps1            # 完整代码检查
│   └── format.ps1           # 代码格式化
├── docs/                    # 文档
│   └── development-workflow.md  # 开发流程文档
├── build.rs                 # 构建脚本（winres 配置）
├── app-manifest.xml         # Windows manifest（管理员权限）
├── rustfmt.toml             # Rust 格式化配置
├── .clippy.toml             # Clippy 配置
├── .editorconfig            # 编辑器配置
├── Makefile.toml            # cargo-make 任务配置
├── Cargo.toml               # 项目配置
└── README.md                # 本文件
```

## 下一步计划

基于当前的核心模块，可以继续实现：

1. **连发引擎** - 实现按键连发逻辑
2. **窗口检测** - 检测 DNF 游戏窗口是否活动
3. **键盘钩子** - 监听触发按键
4. **配置系统** - 可配置的连发设置
5. **用户界面** - 添加 GUI 界面

## 技术特性

- ✅ Trait 抽象设计，方便测试和扩展
- ✅ 零依赖核心逻辑（仅依赖 Windows API）
- ✅ Release 优化（LTO、strip、单编译单元）
- ✅ 游戏专用按键模式（避免聊天框触发）
- ✅ 类型安全的 Rust 实现
- ✅ 自动管理员权限
- ✅ 完整的格式化和 Lint 流程（rustfmt + clippy）
- ✅ 零警告代码质量

## 许可

本项目仅供学习和个人使用。
