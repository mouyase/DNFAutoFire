# DNF AutoFire (Tauri 版)

高性能游戏连发工具，使用 Rust + Tauri + Web 技术栈开发。

## 特性

- **游戏专用按键发送**：游戏能识别，聊天框不会收到输入
- **多键同时连发**：支持同时按住多个按键，依次循环发送
- **低延迟**：33ms 周期（约 30 次/秒），使用高精度定时器
- **窗口检测**：只在 DNF 游戏窗口激活时工作
- **管理员权限**：自动检测并提示以管理员重启
- **美观的 Web UI**：深色主题，完整键盘布局

## 技术栈

- **后端**：Rust + Windows API
- **前端**：HTML + CSS + JavaScript
- **框架**：Tauri 2.x

## 快速开始

### 运行

```bash
# 开发模式
npm run tauri dev

# 构建
npm run tauri build
```

### 使用

1. **以管理员身份运行**（必需）
2. 点击键盘上的按键启用/禁用连发
3. 点击「启动连发」按钮
4. 切换到 DNF 游戏窗口
5. 按住已启用的按键即可连发

## 项目结构

```
├── src/                    # 前端
│   ├── index.html         # 主页面
│   ├── styles.css         # 样式
│   └── main.js            # 逻辑
│
├── src-tauri/              # 后端
│   ├── src/
│   │   ├── lib.rs         # Tauri 命令
│   │   └── core/          # 核心模块
│   │       ├── autofire.rs  # 连发引擎
│   │       ├── keyboard.rs  # 键盘输入
│   │       └── window.rs    # 窗口检测
│   └── Cargo.toml
│
├── KNOWLEDGE.md            # 技术知识库（必读）
└── README.md
```

## 核心原理

详见 [KNOWLEDGE.md](./KNOWLEDGE.md)

### 游戏专用按键

使用 `vkFF + 真实扫描码 + 无 KEYEVENTF_SCANCODE 标志` 的组合：

```rust
KEYBDINPUT {
    wVk: VIRTUAL_KEY(0xFF),      // 无效 VK，聊天框忽略
    wScan: scan_code,             // 真实扫描码，游戏识别
    dwFlags: KEYBD_EVENT_FLAGS(0), // 无标志
    ...
}
```

### 多键连发

1. 使用 `WH_KEYBOARD_LL` 钩子拦截物理按键
2. 首次按下发送正常按键（聊天可用）
3. 后续连发使用游戏专用按键
4. 使用 `AtomicU64` 位掩码管理多键状态

## 配置

### 添加测试窗口

编辑 `src-tauri/src/core/window.rs`：

```rust
const DNF_WINDOW_CLASSES: &[&str] = &[
    "地下城与勇士",  // DNF
    "Notepad",       // 记事本（测试）
];
```

### 调整连发速度

编辑 `src-tauri/src/core/autofire.rs`：

```rust
const KEY_DOWN_MS: u64 = 16;  // 按下时间
const KEY_UP_MS: u64 = 17;    // 释放时间
```

## 常见问题

### Q: 连发没有效果？

1. 确保以管理员权限运行
2. 确保 DNF 窗口在前台
3. 确保已启用要连发的按键

### Q: 如何测试？

将 "Notepad" 添加到窗口类名列表，使用记事本测试。

## 开发

```bash
# 安装依赖
npm install

# 开发模式（热重载）
npm run tauri dev

# 构建发布版
npm run tauri build
```

## 许可

MIT License
