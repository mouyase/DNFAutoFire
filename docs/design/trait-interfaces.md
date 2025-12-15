# DNF AutoFire 抽象接口文档

## 概述

本文档详细描述了 DNF AutoFire 中定义的所有 Trait 抽象接口，这些接口实现了平台无关的核心逻辑，使得系统具备良好的可测试性和可扩展性。

---

## 1. 核心 Traits (`src/core/traits.rs`)

### 1.1 KeyboardDriver

键盘输入驱动抽象，定义了所有键盘输入操作的接口。

```rust
pub trait KeyboardDriver: Send + Sync + Debug {
    /// 将虚拟键码转换为扫描码
    fn vk_to_scan_code(&self, vk: u16) -> u16;

    /// 发送按键按下事件（普通模式）
    fn send_key_down(&self, vk: u16, scan_code: u16);

    /// 发送按键释放事件（普通模式）
    fn send_key_up(&self, vk: u16, scan_code: u16);

    /// 发送完整的按键按下+释放（普通模式）
    fn send_key_press(&self, vk: u16);

    /// 发送游戏专用按键按下（使用 vkFF 技术）
    fn send_game_key_down(&self, scan_code: u16);

    /// 发送游戏专用按键释放（使用 vkFF 技术）
    fn send_game_key_up(&self, scan_code: u16);

    /// 检查指定按键是否处于按下状态
    fn is_key_pressed(&self, vk: u16) -> bool;
}
```

#### 实现说明

| 方法 | Windows 实现 | Mock 实现 |
|------|--------------|-----------|
| `vk_to_scan_code` | 调用 `MapVirtualKeyW` | 返回 vk 本身 |
| `send_key_down` | 调用 `SendInput` | 记录事件 |
| `send_key_up` | 调用 `SendInput` | 记录事件 |
| `send_game_key_down` | 使用 vkFF + 扫描码 | 记录事件 |
| `send_game_key_up` | 使用 vkFF + 扫描码 | 记录事件 |
| `is_key_pressed` | 调用 `GetAsyncKeyState` | 查询内部状态 |

#### 使用示例

```rust
// 生产环境
let keyboard: Arc<dyn KeyboardDriver> = Arc::new(DefaultKeyboardDriver);

// 测试环境
let keyboard = Arc::new(MockKeyboardDriver::new());
keyboard.send_key_press(vk::VK_J);
assert_eq!(keyboard.get_events().len(), 2); // down + up
```

---

### 1.2 WindowDetector

窗口检测器抽象，用于判断目标窗口是否处于活动状态。

```rust
pub trait WindowDetector: Send + Sync + Debug {
    /// 检查目标窗口是否处于活动状态
    fn is_target_active(&self) -> bool;

    /// 获取当前前台窗口的类名
    fn get_foreground_class_name(&self) -> String;
}
```

#### 实现说明

| 方法 | Windows 实现 | Mock 实现 |
|------|--------------|-----------|
| `is_target_active` | 检查窗口类名是否为 "地下城与勇士" | 返回可配置状态 |
| `get_foreground_class_name` | 调用 `GetForegroundWindow` + `GetClassNameW` | 返回模拟类名 |

#### 使用示例

```rust
// 生产环境
let detector = DefaultWindowDetector;
if detector.is_target_active() {
    // 执行连发
}

// 测试环境
let detector = MockWindowDetector::with_state(true, "TestWindow");
assert!(detector.is_target_active());
```

---

### 1.3 KeyboardHook

键盘钩子管理抽象，用于全局键盘事件监听。

```rust
pub trait KeyboardHook: Send + Sync + Debug {
    /// 安装键盘钩子
    fn install(&self, callback: KeyHookCallback) -> Result<(), String>;

    /// 卸载键盘钩子
    fn uninstall(&self) -> Result<(), String>;

    /// 检查钩子是否已安装
    fn is_installed(&self) -> bool;
}
```

#### 关联类型

```rust
/// 按键事件类型
pub enum KeyEventType {
    KeyDown,
    KeyUp,
}

/// 按键事件
pub struct KeyEvent {
    pub vk: u16,
    pub scan_code: u16,
    pub event_type: KeyEventType,
    pub is_injected: bool,
}

/// 钩子处理结果
pub enum HookResult {
    PassThrough,  // 允许事件继续传递
    Consumed,     // 阻止事件继续传递
}

/// 钩子回调函数类型
pub type KeyHookCallback = Box<dyn Fn(KeyEvent) -> HookResult + Send + Sync>;
```

---

### 1.4 PlatformCapabilities

平台能力查询接口，用于运行时检测平台支持的功能。

```rust
pub trait PlatformCapabilities: Send + Sync + Debug {
    /// 是否支持全局键盘钩子
    fn supports_global_hook(&self) -> bool;

    /// 是否支持游戏专用按键发送
    fn supports_game_key_sending(&self) -> bool;

    /// 是否支持窗口检测
    fn supports_window_detection(&self) -> bool;

    /// 获取平台名称
    fn platform_name(&self) -> &'static str;
}
```

---

## 2. Engine 模块类型 (`src/engine/`)

### 2.1 KeyAction

单个按键动作的定义。

```rust
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "type", content = "data")]
pub enum KeyAction {
    /// 按下按键（普通模式，聊天框可识别）
    Press { vk: u16 },

    /// 按下按键（游戏专用模式，聊天框不识别）
    GamePress { vk: u16 },

    /// 按住按键不放
    Hold { vk: u16 },

    /// 释放按键
    Release { vk: u16 },

    /// 延迟指定毫秒
    Delay { ms: u64 },
}
```

#### 便捷构造方法

```rust
impl KeyAction {
    pub fn press(vk: u16) -> Self;
    pub fn game_press(vk: u16) -> Self;
    pub fn hold(vk: u16) -> Self;
    pub fn release(vk: u16) -> Self;
    pub fn delay(ms: u64) -> Self;
}
```

---

### 2.2 TriggerRule

触发规则定义，描述什么条件下执行什么动作。

```rust
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TriggerRule {
    /// 规则唯一 ID
    pub id: String,

    /// 触发此规则的按键列表（任一按键按下即触发）
    pub trigger_keys: Vec<u16>,

    /// 要执行的动作序列
    pub actions: Vec<KeyAction>,

    /// 是否循环执行
    pub repeat: bool,

    /// 循环间隔（毫秒），仅 repeat=true 时有效
    pub interval_ms: Option<u64>,

    /// 规则是否启用
    pub enabled: bool,
}
```

#### 预设规则构造方法

```rust
impl TriggerRule {
    /// 简单连发：按住触发键 -> 循环按下指定键
    pub fn simple_repeat(vk: u16, interval_ms: u64) -> Self;

    /// 延迟连发：首次延迟 -> 循环按下指定键
    pub fn delayed_repeat(vk: u16, initial_delay_ms: u64, interval_ms: u64) -> Self;

    /// 组合连发：按住触发键 -> 循环按下多个键
    pub fn combo_repeat(trigger_vk: u16, combo_vks: Vec<u16>, interval_ms: u64) -> Self;

    /// 自定义规则
    pub fn custom(id: &str, trigger_keys: Vec<u16>, actions: Vec<KeyAction>, repeat: bool) -> Self;
}
```

---

### 2.3 ActionExecutor

动作执行器，使用泛型实现依赖注入。

```rust
pub struct ActionExecutor<K: KeyboardDriver, W: WindowDetector> {
    keyboard: Arc<K>,
    window: Arc<W>,
}

impl<K: KeyboardDriver, W: WindowDetector> ActionExecutor<K, W> {
    /// 创建执行器
    pub fn new(keyboard: Arc<K>, window: Arc<W>) -> Self;

    /// 执行单个动作
    pub fn execute_action(&self, action: &KeyAction);

    /// 执行动作序列
    pub fn execute_sequence<F>(&self, actions: &[KeyAction], should_continue: F) -> ExecutionResult
    where
        F: FnMut() -> bool;

    /// 执行触发规则（单次）
    pub fn execute_rule_once<F>(&self, rule: &TriggerRule, should_continue: F) -> ExecutionResult
    where
        F: FnMut() -> bool;

    /// 执行触发规则（循环）
    pub fn execute_rule_loop<F>(&self, rule: &TriggerRule, should_continue: F) -> ExecutionResult
    where
        F: FnMut() -> bool;
}
```

#### 执行结果

```rust
pub enum ExecutionResult {
    Success,        // 执行成功
    Interrupted,    // 被中断（如按键释放）
    WindowInactive, // 窗口不活动
}
```

---

### 2.4 ExecutionContext

执行上下文，用于跟踪执行状态。

```rust
pub struct ExecutionContext {
    /// 当前是否应该继续执行
    pub should_run: bool,

    /// 当前按下的触发键
    pub active_keys: Vec<u16>,
}

impl ExecutionContext {
    pub fn new() -> Self;
    pub fn is_key_active(&self, vk: u16) -> bool;
    pub fn key_down(&mut self, vk: u16);
    pub fn key_up(&mut self, vk: u16);
}
```

---

### 2.5 Profile

用户配置档案。

```rust
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Profile {
    pub id: String,
    pub name: String,
    pub rules: Vec<TriggerRule>,
    pub created_at: u64,
    pub updated_at: u64,
}

impl Profile {
    pub fn new(name: &str) -> Self;
    pub fn with_rules(name: &str, rules: Vec<TriggerRule>) -> Self;
    pub fn default_profile() -> Self;
    pub fn add_rule(&mut self, rule: TriggerRule);
    pub fn remove_rule(&mut self, rule_id: &str) -> bool;
    pub fn find_rules_for_key(&self, vk: u16) -> Vec<&TriggerRule>;
}
```

---

### 2.6 ProfileManager

配置档案管理器。

```rust
pub struct ProfileManager {
    profiles: Vec<Profile>,
    active_profile_id: Option<String>,
}

impl ProfileManager {
    pub fn new() -> Self;
    pub fn profiles(&self) -> &[Profile];
    pub fn active_profile(&self) -> Option<&Profile>;
    pub fn set_active(&mut self, profile_id: &str) -> bool;
    pub fn add_profile(&mut self, profile: Profile) -> String;
    pub fn create_profile(&mut self, name: &str) -> String;
    pub fn remove_profile(&mut self, profile_id: &str) -> bool;
}
```

---

## 3. Mock 实现

### 3.1 MockKeyboardDriver

```rust
pub struct MockKeyboardDriver {
    events: Arc<Mutex<Vec<RecordedKeyEvent>>>,
    pressed_keys: Arc<Mutex<HashSet<u16>>>,
}

impl MockKeyboardDriver {
    pub fn new() -> Self;
    pub fn get_events(&self) -> Vec<RecordedKeyEvent>;
    pub fn clear_events(&self);
    pub fn simulate_key_press(&self, vk: u16);
    pub fn simulate_key_release(&self, vk: u16);
}

pub enum RecordedKeyEvent {
    KeyDown { vk: u16, scan_code: u16 },
    KeyUp { vk: u16, scan_code: u16 },
    GameKeyDown { scan_code: u16 },
    GameKeyUp { scan_code: u16 },
}
```

### 3.2 MockWindowDetector

```rust
pub struct MockWindowDetector {
    is_active: Arc<RwLock<bool>>,
    class_name: Arc<RwLock<String>>,
}

impl MockWindowDetector {
    pub fn new() -> Self;  // 默认活动状态
    pub fn with_state(is_active: bool, class_name: &str) -> Self;
    pub fn set_active(&self, active: bool);
    pub fn set_class_name(&self, name: &str);
}
```

### 3.3 便捷检测器

```rust
/// 永远返回活动状态
pub struct AlwaysActiveDetector;

/// 永远返回非活动状态
pub struct NeverActiveDetector;
```

---

## 4. 序列化支持

所有数据类型都实现了 `Serialize` 和 `Deserialize`，支持 JSON 格式存储：

```rust
// 序列化示例
let rule = TriggerRule::simple_repeat(vk::VK_J, 33);
let json = serde_json::to_string(&rule)?;

// 反序列化示例
let rule: TriggerRule = serde_json::from_str(&json)?;
```

---

## 5. 线程安全

所有核心 Trait 都要求实现 `Send + Sync`：

- `KeyboardDriver: Send + Sync`
- `WindowDetector: Send + Sync`
- `KeyboardHook: Send + Sync`

这确保了这些接口的实现可以安全地在多线程环境中使用。
