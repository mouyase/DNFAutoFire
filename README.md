# Python Interception 包使用文档

## 简介

Interception 是一个用于模拟和控制鼠标键盘输入的 Python 包。它可以:

- 模拟鼠标移动、点击
- 模拟键盘按键输入  
- 捕获鼠标和键盘事件
- 支持人性化的鼠标移动曲线

## 主要功能

### 1. 鼠标控制

#### 移动鼠标
```python
# 移动到绝对坐标
move_to(x, y)  

# 相对当前位置移动
move_relative(x=100, y=0)  # 向右移动100像素

# 获取当前鼠标位置
pos = mouse_position()  # 返回 (x,y) 坐标元组
```

#### 鼠标点击
```python
# 在当前位置点击
click()  # 默认左键单击
right_click()  # 右键单击
left_click()  # 左键单击

# 移动并点击
click(800, 600)  # 移动到(800,600)并左键单击
click(800, 600, button="right")  # 移动并右键点击

# 多次点击
click(clicks=2)  # 双击
click(clicks=3, interval=0.1)  # 三击,间隔0.1秒
```

### 2. 键盘控制

#### 按键操作
```python
# 按下并释放按键
press("a")  # 输入字母a
press("enter")  # 回车
press("ctrl")  # ctrl键

# 连续按键
press("a", presses=3)  # 连续按3次a
press("space", presses=2, interval=0.5)  # 按2次空格,间隔0.5秒
```

#### 文本输入
```python
# 输入文本
write("Hello World")  # 逐字符输入
write("test", interval=0.1)  # 设置字符间隔时间
```

### 3. 组合键操作

使用 `hold_key` 上下文管理器实现组合键:

```python
# Ctrl+C 复制
with hold_key("ctrl"):
    press("c")

# Shift+A 输入大写A  
with hold_key("shift"):
    press("a")
```

### 4. 事件捕获

```python
# 捕获键盘事件,按ESC退出
capture_keyboard()

# 捕获鼠标左键点击,按ESC退出  
capture_mouse()
```

## 注意事项

1. 使用前需要安装 interception 驱动
2. 某些操作可能需要管理员权限
3. 建议在操作之间添加适当延时
4. 鼠标坐标使用屏幕绝对坐标系统

## 高级功能

### 人性化鼠标移动

可以通过 `curve_params` 参数实现更自然的鼠标移动轨迹:

```python
from interception import beziercurve

# 创建曲线参数
params = beziercurve.BezierCurveParams()

# 使用曲线移动
move_to(800, 600, curve_params=params)
```

这将使鼠标移动更接近人类操作,不会显得太过机械。

## 错误处理

包中定义了以下主要异常:

- DriverNotFoundError: 驱动未安装
- UnknownButtonError: 未知的鼠标按键
- UnknownKeyError: 未知的键盘按键

建议使用 try-except 进行异常处理:

```python
try:
    move_to(100, 100)
except exceptions.DriverNotFoundError:
    print("请先安装 interception 驱动")
```

## 安装说明

1. 首先安装 Python 包:
```bash
pip install interception
```

2. 下载并安装 interception 驱动:
   - 访问 [interception 驱动下载页面](https://github.com/oblitum/Interception/releases)
   - 下载并安装最新版本的驱动
   - 重启电脑使驱动生效

## 许可证

本项目采用 MIT 许可证。详见 LICENSE 文件。

## 贡献

欢迎提交 Issue 和 Pull Request 来帮助改进这个项目。

## 联系方式

如有问题或建议,请通过以下方式联系:

- 提交 GitHub Issue
- 发送邮件至: [your-email@example.com] 