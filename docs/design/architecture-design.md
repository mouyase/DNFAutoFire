# DNF AutoFire 架构设计文档

## 1. 系统概述

DNF AutoFire 是一个为 DNF（地下城与勇士）设计的连发辅助工具，基于 Tauri 2 + React 19 技术栈构建。

### 1.1 核心特性

- **游戏专用按键发送**：使用 vkFF + 真实扫描码技术，确保按键被游戏识别但不会影响聊天框
- **多键同时连发**：支持多个按键同时触发连发
- **窗口检测**：仅在 DNF 窗口激活时工作
- **跨平台开发**：支持在 macOS/Linux 上进行 UI 开发测试

### 1.2 技术栈

| 层级 | 技术 |
|------|------|
| 前端 | React 19 + Vite 7 + TailwindCSS 4 |
| 后端 | Tauri 2 + Rust |
| 构建 | pnpm + Cargo |
| 代码质量 | oxlint + ESLint + Clippy |

---

## 2. 架构分层

```
┌─────────────────────────────────────────────────────────────┐
│                      React Frontend                          │
│  ┌─────────────┐  ┌─────────────┐  ┌─────────────────────┐  │
│  │ Components  │  │    Hooks    │  │        Lib          │  │
│  │  keyboard   │  │ useAutofire │  │  tauri commands     │  │
│  │  layout     │  │  useAdmin   │  │  vk codes           │  │
│  └─────────────┘  └─────────────┘  └─────────────────────┘  │
└─────────────────────────────────────────────────────────────┘
                              │
                              │ Tauri IPC (invoke)
                              ▼
┌─────────────────────────────────────────────────────────────┐
│                      Tauri Commands (lib.rs)                 │
│  ┌─────────────────────────────────────────────────────────┐│
│  │ get_enabled_keys, toggle_key, start_autofire, ...       ││
│  └─────────────────────────────────────────────────────────┘│
└─────────────────────────────────────────────────────────────┘
                              │
                              │ Rust 内部调用
                              ▼
┌─────────────────────────────────────────────────────────────┐
│                      Core Modules                            │
│  ┌──────────────┐  ┌──────────────┐  ┌──────────────┐       │
│  │   autofire   │  │   keyboard   │  │    window    │       │
│  │   Engine     │  │    Driver    │  │   Detector   │       │
│  └──────────────┘  └──────────────┘  └──────────────┘       │
│           │                │                 │               │
│           └────────────────┴─────────────────┘               │
│                            │                                 │
│                    Trait 抽象层                               │
│  ┌─────────────────────────────────────────────────────────┐│
│  │ KeyboardDriver │ WindowDetector │ KeyboardHook          ││
│  └─────────────────────────────────────────────────────────┘│
└─────────────────────────────────────────────────────────────┘
                              │
                              │ 平台实现
                              ▼
┌─────────────────────────────────────────────────────────────┐
│                   Platform Layer                             │
│  ┌─────────────────────────┐  ┌─────────────────────────┐   │
│  │     Windows 实现         │  │      Mock 实现          │   │
│  │  - SendInput API        │  │  - 测试用 Mock 驱动      │   │
│  │  - WH_KEYBOARD_LL Hook  │  │  - 事件记录验证          │   │
│  │  - GetForegroundWindow  │  │  - 状态模拟              │   │
│  └─────────────────────────┘  └─────────────────────────┘   │
└─────────────────────────────────────────────────────────────┘
                              │
                              │ 未来扩展
                              ▼
┌─────────────────────────────────────────────────────────────┐
│                      Engine Modules                          │
│  ┌──────────────┐  ┌──────────────┐  ┌──────────────┐       │
│  │    Action    │  │   Executor   │  │   Profile    │       │
│  │  KeyAction   │  │ ActionExec   │  │ ProfileMgr   │       │
│  │ TriggerRule  │  │ ExecContext  │  │   Profile    │       │
│  └──────────────┘  └──────────────┘  └──────────────┘       │
└─────────────────────────────────────────────────────────────┘
```

---

## 3. 模块详解

### 3.1 Core 模块 (`src-tauri/src/core/`)

#### 3.1.1 traits.rs - 抽象接口定义

定义平台无关的核心能力接口：

- `KeyboardDriver` - 键盘输入驱动
- `WindowDetector` - 窗口检测器
- `KeyboardHook` - 键盘钩子管理
- `PlatformCapabilities` - 平台能力查询

#### 3.1.2 keyboard.rs - 键盘驱动

- **Windows 实现**：使用 `SendInput` API 发送按键
- **Mock 实现**：记录事件用于测试验证

#### 3.1.3 window.rs - 窗口检测

- **Windows 实现**：使用 `GetForegroundWindow` + `GetClassNameW`
- **Mock 实现**：可配置的状态模拟

#### 3.1.4 autofire.rs - 连发引擎

核心连发逻辑，包括：
- 键盘钩子注册
- 连发循环控制
- 多键状态管理

### 3.2 Engine 模块 (`src-tauri/src/engine/`)

#### 3.2.1 action.rs - 动作定义

```rust
pub enum KeyAction {
    Press { vk: u16 },      // 普通按键
    GamePress { vk: u16 },  // 游戏专用按键
    Hold { vk: u16 },       // 按住
    Release { vk: u16 },    // 释放
    Delay { ms: u64 },      // 延迟
}

pub struct TriggerRule {
    pub trigger_keys: Vec<u16>,
    pub actions: Vec<KeyAction>,
    pub repeat: bool,
    pub interval_ms: Option<u64>,
}
```

#### 3.2.2 executor.rs - 动作执行器

使用依赖注入模式，支持任意 `KeyboardDriver` 和 `WindowDetector` 实现：

```rust
pub struct ActionExecutor<K: KeyboardDriver, W: WindowDetector> {
    keyboard: Arc<K>,
    window: Arc<W>,
}
```

#### 3.2.3 profile.rs - 配置档案

支持多配置切换：

```rust
pub struct Profile {
    pub id: String,
    pub name: String,
    pub rules: Vec<TriggerRule>,
}

pub struct ProfileManager {
    profiles: Vec<Profile>,
    active_profile_id: Option<String>,
}
```

---

## 4. 设计原则

### 4.1 依赖注入

所有核心组件通过 Trait 抽象，支持：
- 生产环境使用真实实现
- 测试环境使用 Mock 实现
- 跨平台开发时无缝切换

### 4.2 单一职责

每个模块只负责一个明确的功能：
- `keyboard` - 只负责键盘输入
- `window` - 只负责窗口检测
- `executor` - 只负责动作执行

### 4.3 状态管理

- **Rust 层**：作为状态的唯一真实来源（Single Source of Truth）
- **React 层**：通过 Tauri 命令同步状态，仅负责 UI 渲染

### 4.4 跨平台兼容

使用条件编译 `#[cfg(windows)]` 分离平台特定代码：

```rust
#[cfg(windows)]
mod windows_impl { ... }

#[cfg(not(windows))]
mod mock_impl { ... }
```

---

## 5. 数据流

### 5.1 按键启用流程

```
用户点击按键 → React 组件 → useAutofire Hook
    → invoke("toggle_key") → Tauri Command
    → AppState.enabled_keys.insert() → AutoFireEngine.set_keys()
    → 返回新状态 → React 更新 UI
```

### 5.2 连发执行流程

```
启动连发 → 注册键盘钩子 → 用户按下按键
    → 钩子捕获 → 检查是否启用 → 检查窗口活动
    → 开始连发循环 → 发送游戏按键
    → 用户释放按键 → 停止连发循环
```

---

## 6. 扩展性设计

### 6.1 复杂按键序列

`TriggerRule` 支持定义复杂的按键序列：

```rust
TriggerRule::delayed_repeat(vk, initial_delay, interval);
TriggerRule::combo_repeat(trigger, combo_keys, interval);
```

### 6.2 多配置档案

`ProfileManager` 支持：
- 创建/删除配置
- 切换活动配置
- 配置序列化/反序列化

### 6.3 未来扩展点

1. **条件触发**：根据游戏状态触发不同动作
2. **宏录制**：录制用户操作序列
3. **云同步**：配置云端备份
