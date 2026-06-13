# L 键 Hook 连发实验

这是一个仅支持 Windows 的 Rust 命令行实验程序，用来验证 DNF 里是否能走真实的 `按住物理 L -> 游戏只收到程序连发 L` 路径。

程序使用低级键盘 hook：`WH_KEYBOARD_LL`。

- 物理键盘按下的 `L`：作为触发信号，并且被拦截，不让它继续进入游戏。
- 程序自己发送的 `L`：使用 `SendInput + 扫描码` 发送，并且放行给游戏。

## 为什么要这样测

DNF 里真正有用的指标不是 Windows 收到了多少按键事件，而是游戏内技能实际释放了多少次。

如果不拦截物理 `L`，游戏可能同时收到：

- 原始物理 `L` 按下/抬起
- Windows 自动重复产生的 `L`
- 程序连发出来的 synthetic `L`

这样测出来的技能次数就混在一起了，不知道到底是谁产生的。本实验的目标是把物理 `L` 从游戏输入里拿掉，让游戏尽量只看到程序发送的 `L`。

## 构建

在仓库根目录运行：

```powershell
$env:Path = "$env:USERPROFILE\.cargo\bin;$env:Path"
cargo build --manifest-path experiments\l-hook-fire-probe\Cargo.toml
```

## 第一步：只测试拦截是否有效

如果状态里 `看到L事件` 一直是 0，先不要继续测连发。请先用“观察所有按键”模式确认 hook 是否能看到任何键盘事件：

```powershell
$env:Path = "$env:USERPROFILE\.cargo\bin;$env:Path"
cargo run --manifest-path experiments\l-hook-fire-probe\Cargo.toml -- --mode observe-all
```

或者双击 `run.bat` 后选择第 `3` 项。

观察所有按键模式不会拦截，也不会连发。建议先在记事本或 PowerShell 窗口前台按 `A`、`L`、空格：

- 如果 `看到全部键盘事件` 会增加，说明 hook 本身能工作。
- 如果普通窗口里按键也完全不增加，说明 hook 没有收到系统键盘事件，需要优先检查权限、杀软/安全软件、系统限制或程序是否真的启动成功。
- 如果普通窗口能看到，但切到 DNF 后看不到，说明问题集中在 DNF/权限/反作弊/输入路径。

先不要连发，只验证 DNF 是否收不到物理 `L`：

```powershell
$env:Path = "$env:USERPROFILE\.cargo\bin;$env:Path"
cargo run --manifest-path experiments\l-hook-fire-probe\Cargo.toml -- --mode suppress-only
```

然后切到 DNF，按住物理 `L`。

预期结果：

- 程序窗口里 `看到L事件`、`拦截物理按下`、`拦截自动重复` 会增加。
- DNF 里绑定到 `L` 的技能不应该释放。

如果这里没有效果，优先检查：

1. DNF 如果是管理员权限运行，本程序也要用管理员权限运行。
2. 程序状态行里的 `前台窗口` 是否真的是 DNF。
3. `看到L事件` 是否增加：如果不增加，说明 hook 没收到 L。
4. `拦截物理按下` 是否增加：如果增加但 DNF 仍然响应 L，说明 DNF 可能没有走普通键盘队列路径，可能使用了 Raw Input / DirectInput / 反作弊过滤后的输入路径。

## 第二步：拦截物理 L 并连发 synthetic L

确认 `suppress-only` 能让 DNF 收不到物理 `L` 后，再测试连发：

```powershell
$env:Path = "$env:USERPROFILE\.cargo\bin;$env:Path"
cargo run --manifest-path experiments\l-hook-fire-probe\Cargo.toml -- --mode normal --rate 120 --down-ms 2 --up-ms 6
```

然后切到 DNF，按住物理 `L`，做固定时长或固定一轮技能的测试。

## 参数

- `--mode normal`：默认模式。拦截物理 `L`，同时发送 synthetic `L` 连发。
- `--mode suppress-only`：只拦截物理 `L`，不发送 synthetic `L`。用于诊断 hook 是否真的能挡住 DNF。
- `--mode observe-all`：观察所有键盘事件，不拦截、不连发。用于诊断 hook 是否能看到键盘事件。
- `--mode test-send`：倒计时 3 秒后直接发送指定次数的 `L`，不需要按住物理 `L`。用于判断 `SendInput` 是否对当前前台窗口有效。
- `--multi-jlh`：多键实验模式。只处理 `J`、`L`、`H` 三个键；物理键会被拦截，程序按队列规则发送 synthetic `J/L/H`。
- `--send-mode scancode`：用扫描码发送，默认值，接近旧 AHK 的思路。
- `--send-mode vk`：用虚拟键 `VK_L` 发送。
- `--send-mode mixed`：同时填写 `VK_L` 和扫描码，并带 `KEYEVENTF_SCANCODE`。
- `--send-mode game`：旧 `legacy/dev` 里的游戏专用模式：`wVk=0xFF + 真实扫描码`，不带 `KEYEVENTF_SCANCODE`；抬起时只带 `KEYEVENTF_KEYUP`。旧代码注释说这种方式“游戏能识别，聊天框不识别”。
- `--max-speed`：极速模式。按住物理 `L` 时不再按 `--rate` 节流，而是按 `down-ms/up-ms` 尽快循环发送。
- `--rate`：目标连发次数/秒，默认 `60`。
- `--down-ms`：synthetic `L` 按下保持多久，默认 `8` 毫秒。
- `--up-ms`：synthetic `L` 抬起后等待多久，默认 `8` 毫秒。
- `--test-count`：`test-send` 模式下直接发送多少次 `L`，默认 `20`。

## 如果能看到 L 事件，但游戏没有反应

这说明 hook/权限基本通了，下一步要确认 `SendInput` 是否被目标窗口接受。

先用 `test-send` 测普通窗口：

```powershell
$env:Path = "$env:USERPROFILE\.cargo\bin;$env:Path"
cargo run --manifest-path experiments\l-hook-fire-probe\Cargo.toml -- --mode test-send --send-mode scancode --test-count 20
```

程序会倒计时 3 秒。请在倒计时期间切到记事本。

- 如果记事本里出现 `l`，说明 `SendInput` 本身能发出去。
- 如果记事本也没有任何输入，说明当前发送构造有问题，需要换发送模式。

然后依次试：

```powershell
cargo run --manifest-path experiments\l-hook-fire-probe\Cargo.toml -- --mode test-send --send-mode vk --test-count 20
cargo run --manifest-path experiments\l-hook-fire-probe\Cargo.toml -- --mode test-send --send-mode mixed --test-count 20
cargo run --manifest-path experiments\l-hook-fire-probe\Cargo.toml -- --mode test-send --send-mode game --test-count 20
```

如果记事本能收到，但 DNF 收不到，说明 DNF 很可能过滤或不接受这个级别的 `SendInput`。

如果某个发送方式在 DNF 有反应，再把正常连发切到同样的发送方式：

```powershell
cargo run --manifest-path experiments\l-hook-fire-probe\Cargo.toml -- --mode normal --send-mode vk --rate 120 --down-ms 2 --up-ms 6
```

根据 `legacy/dev` 旧 Rust 代码，最值得优先测试的是：

```powershell
cargo run --manifest-path experiments\l-hook-fire-probe\Cargo.toml -- --mode test-send --send-mode game --test-count 20
```

如果 `game` 模式在 DNF 有反应，正常连发也应该优先使用：

```powershell
cargo run --manifest-path experiments\l-hook-fire-probe\Cargo.toml -- --mode normal --send-mode game --rate 120 --down-ms 16 --up-ms 17
```

这里的 `16ms/17ms` 也是旧 `legacy/dev` 连发线程使用的节奏。

## 极速发射测试

如果 `test-send + game` 能让 DNF 放技能，可以开始测极速模式：

```powershell
cargo run --manifest-path experiments\l-hook-fire-probe\Cargo.toml -- --mode normal --send-mode game --max-speed --down-ms 1 --up-ms 1
```

或者双击 `run.bat` 后选第 `5` 项。

建议按下面顺序记录 DNF 技能实际释放次数：

| down-ms | up-ms | 说明 |
|---:|---:|---|
| 1 | 1 | 默认极速起点 |
| 0 | 0 | 极限发送，可能太快导致游戏吞输入 |
| 2 | 1 | 稍微拉长按下时间 |
| 4 | 1 | 给游戏更多机会识别 down |
| 8 | 1 | 如果太快吞输入，试这个 |
| 16 | 17 | 旧版稳定节奏对照 |

最终以 DNF 里技能实际次数为准，不以 `已发送连发` 最大为准。游戏可能在太快时吞输入，所以最高发送频率不一定是最高有效释放次数。

## 60 次/秒稳定 Demo

如果只是想先试一个相对舒服、不容易掉帧的配置，可以直接用：

```powershell
cargo run --manifest-path experiments\l-hook-fire-probe\Cargo.toml -- --mode normal --send-mode game --rate 60 --down-ms 8 --up-ms 8
```

或者双击 `run.bat` 后选第 `6` 项。

这个配置含义是：

- 使用旧版可进游戏的 `game` 发送模式。
- 目标 60 次/秒。
- 每次按下保持 8ms，抬起后等待 8ms。
- 不使用 `--max-speed`，所以不会无节制制造输入风暴。

### 当前好用节奏的精确定义

当前已确认好用的基线不是“按下占一整帧、抬起再占一整帧”，而是每个 60Hz 发送周期内完成一次完整 tap：

```text
约 0ms：发送 key down
约 0ms ~ 8ms：保持按下
约 8ms：发送 key up
约 8ms ~ 16.67ms：保持抬起，也就是 gap
下一个 60Hz 周期：再发送下一次 tap
```

所以它可以理解为：

```text
一帧内完成一次 tap：down 8ms -> up/gap 约 8~9ms
```

这点对后续帧调度器很重要。如果把 `down` 和 `up` 各拆成一个完整 60Hz 帧，就会变成约 30 次/秒，和当前实测好用的 60 次/秒基线不是同一种节奏。

### 已确认的实验事实

2026-06-13 实测：

- 游戏：地下城与勇士：创新世纪。
- 权限：程序以管理员权限运行。
- 发送模式：`game`，也就是旧版 `legacy/dev` 使用的 `wVk=0xFF + 真实扫描码`。
- 频率：`60` 次/秒。
- 参数：`down-ms=8`，`up-ms=8`。
- 结果：DNF 内效果非常好。

这个配置应作为后续正式产品的默认候选基线。继续优化时，优先围绕它做小范围参数扫描，而不是直接使用 `1ms/1ms` 这类极限输入。

## J/L/H 多键队列 Demo

当前多键第一版固定只支持 `J`、`L`、`H` 三个键。启动方式：

```powershell
cargo run --manifest-path experiments\l-hook-fire-probe\Cargo.toml -- --mode normal --send-mode game --multi-jlh --rate 60 --down-ms 8 --up-ms 8
```

或者双击 `run.bat` 后选第 `7` 项。

规则：

- 全局输入预算仍然是 `60` 次/秒，不是每个键各 60 次/秒。
- 只处理 `J`、`L`、`H`，其他键不拦截、不连发。
- 首次按下一个目标键时，它会进入队首；系统自动重复 keydown 不会重复入队。
- 后按下的键排在前面，并且下一个发送周期会从队首重新开始。
- 松开某个键时，它会从队列里移除。
- 发送线程按当前队列从前到后循环，每个发送周期只发一个 tap。

示例：

```text
按下 J      -> 队列 [J]      -> 输出 J J J J
继续按 L    -> 队列 [L, J]   -> 输出 L J L J
继续按 H    -> 队列 [H, L, J]-> 输出 H L J H L J
松开 L      -> 队列 [H, J]   -> 输出 H J H J
```

如果按下顺序是 `J -> L -> H`，下一个发送周期开始后的顺序就是 `H -> L -> J`。这个 Demo 暂时不判断技能 CD，只验证多键同时按住时的触发顺序和 60 次/秒共享预算。

### 多键之间是否有间隔

当前 `--multi-jlh --rate 60 --down-ms 8 --up-ms 8` 不是“上一个键抬起的同时按下下一个键”。每个键仍然按完整 tap 执行：

```text
J down
等待 8ms
J up
等待约 8~9ms
L down
等待 8ms
L up
等待约 8~9ms
```

也就是说，`J up` 到 `L down` 之间会有一个短 gap。这个短 gap 是当前成功基线的一部分，后续如果做帧队列或时间轴调度，默认应先保留这种节奏，而不是直接改成 `J up + L down` 同时发生。

## 状态行怎么看

程序每秒输出一行中文状态：

- `前台窗口`：当前前台窗口标题，用来确认你是否真的切到了 DNF。
- `看到全部键盘事件`：只在 `observe-all` 模式下用于诊断，表示 hook 看到的所有键盘事件数。
- `最近键`：最近一次 hook 看到的按键，包含虚拟键码、扫描码、消息类型、是否注入。
- `看到目标键事件`：hook 看到的目标键事件总数；单键模式是 `L`，多键模式是 `J/L/H`。
- `已发送连发`：程序完成的 synthetic `L` tap 次数。
- `看到自己注入事件`：hook 看到的本程序 synthetic `L` 事件数。一次 tap 通常会产生 down/up 两个事件。
- `拦截物理按下`：被拦截的首次物理 `L` down。
- `拦截自动重复`：按住物理 `L` 后，被拦截的系统自动重复 down。
- `拦截物理抬起`：被拦截的物理 `L` up。
- `其他注入`：其他程序注入的 `L`。
- `发送失败`：`SendInput` 调用失败次数。
- `当前按住`：程序当前认为哪些目标物理键正在按住。
- `多键队列`：多键模式下的当前发送队列，越靠左越先发。

最终判断标准仍然是：DNF 技能实际释放次数，而不是 `已发送连发`。

## 安全说明

- 普通模式只处理 `L`；`--multi-jlh` 模式只处理 `J/L/H`，不会处理其他键。
- 用 `Ctrl+C` 退出。
- 如果输入看起来卡住，先松开 `L`，再停止程序，然后切到普通窗口测试键盘。
- 游戏或反作弊可能忽略、过滤、阻止或特殊处理用户态 hook 和注入输入。本程序只是测量实验，不是最终产品实现。
