# DNF AutoFire 技术笔记

记录开发过程中验证过的关键技术点。

## 核心发现

### 1. 按键发送方式（最关键！）

经过测试，发现三种 SendInput 组合的效果：

| 方式 | wVk | wScan | dwFlags | 游戏识别 | 聊天框识别 |
|------|-----|-------|---------|----------|------------|
| 方式1 | 0xFF | 真实sc | KEYEVENTF_SCANCODE | ✅ 是 | ✅ 是 |
| **方式2** | **0xFF** | **真实sc** | **无标志** | **✅ 是** | **❌ 否** |
| 方式3 | 真实vk | 0xFF | KEYEVENTF_SCANCODE | ❌ 否 | ❌ 否 |

**结论**：使用 **方式2**（vkFF + 真实sc + 无 KEYEVENTF_SCANCODE 标志）可以实现：
- 游戏能识别（读取 wScan 字段）
- 聊天框/输入法不识别（wVk=0xFF 无效）

```rust
// 游戏专用按键发送
fn send_key_game_only(sc: u16, key_up: bool) {
    let flags = if key_up {
        KEYEVENTF_KEYUP
    } else {
        KEYBD_EVENT_FLAGS(0)  // 关键：不设置 KEYEVENTF_SCANCODE
    };

    let input = INPUT {
        r#type: INPUT_KEYBOARD,
        Anonymous: INPUT_0 {
            ki: KEYBDINPUT {
                wVk: VIRTUAL_KEY(0xFF),  // 无效虚拟键码
                wScan: sc,                // 真实扫描码
                dwFlags: flags,
                time: 0,
                dwExtraInfo: 0,
            },
        },
    };

    unsafe {
        SendInput(&[input], std::mem::size_of::<INPUT>() as i32);
    }
}
```

### 2. 键盘钩子机制

使用 `WH_KEYBOARD_LL` 低级键盘钩子：

1. **拦截原始按键**：阻止系统重复键到达游戏/聊天框
2. **首次按下**：发送一次正常按键（让聊天框能收到一个字符）
3. **系统重复**：全部拦截（返回 LRESULT(1)）
4. **连发**：用方式2发送（游戏识别，聊天框不识别）

```rust
// 检测是否是注入的按键（避免处理自己发送的）
let is_injected = (kb.flags.0 & LLKHF_INJECTED.0) != 0
    || (kb.flags.0 & LLKHF_LOWER_IL_INJECTED.0) != 0;

if is_keydown && !is_injected {
    if 首次按下 {
        // 发送正常按键（聊天框能识别）
        send_key_normal(vk, sc, false);
    }
    // 拦截原始按键
    return LRESULT(1);
}
```

### 3. 多键连发逻辑

按住多个键时，每个周期**依次**发送每个键：
- 按住 A：A, A, A, A...
- 按住 A+B：A, B, A, B, A, B...
- 按住 A+B+C：A, B, C, A, B, C...

使用位掩码（AtomicU32）管理多键状态，简单可靠。

### 4. 时序参数

经过测试，最佳间隔：
- **按下时长**：16ms
- **释放后间隔**：17ms
- **总周期**：33ms（约 30次/秒）

注意：发送太快可能导致游戏检测不到，33ms 是较好的平衡点。

### 5. 管理员权限

**必须以管理员身份运行**，否则 SendInput 无法发送到游戏进程。

在 `build.rs` 中嵌入清单请求管理员权限：
```rust
res.set_manifest(r#"
<requestedExecutionLevel level="requireAdministrator" uiAccess="false"/>
"#);
```

### 6. AHK 原版分析

AHK 使用的格式 `vkFFsc{XX}` 对应我们的方式1（有 SCANCODE 标志），
但我们发现**不设置 SCANCODE 标志**效果更好。

AHK 的 `{Blind}` 和 `DownTemp` 主要用于修饰键处理，不影响聊天框识别。

## 文件说明

- `test_final.rs` - 单键连发，验证核心机制
- `test_multi2.rs` - 多键连发，最终可用版本
- `test_vksc.rs` - vk/sc 组合测试
- `test_noscanflag.rs` - dwFlags 组合测试（发现方式2）

## 待完成

- [ ] 整合到主程序
- [ ] GUI 界面（按键配置）
- [ ] 配置文件保存/加载
- [ ] 预设管理
- [ ] 窗口检测（只在 DNF 窗口激活时工作）
