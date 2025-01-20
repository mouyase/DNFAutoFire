import keyboard
import win32gui
import win32api
import win32con
import threading
import time
from dataclasses import dataclass
from typing import Dict, List, Optional
from enum import Enum

# 常量定义
TARGET_WINDOWS = ["地下城与勇士", "DNF"]
KEY_LOOP_INTERVAL = 0.001  # 1ms的循环间隔

# 预定义需要连发的按键及其虚拟键码
MONITORED_KEYS = {
    'A': {'name': 'A键', 'vk': 0x41, 'scan': 0x1E},
    'B': {'name': 'B键', 'vk': 0x42, 'scan': 0x30},
    'C': {'name': 'C键', 'vk': 0x43, 'scan': 0x2E},
    'D': {'name': 'D键', 'vk': 0x44, 'scan': 0x20},
    'E': {'name': 'E键', 'vk': 0x45, 'scan': 0x12},
    'F': {'name': 'F键', 'vk': 0x46, 'scan': 0x21},
    'G': {'name': 'G键', 'vk': 0x47, 'scan': 0x22},
    'H': {'name': 'H键', 'vk': 0x48, 'scan': 0x23},
    'I': {'name': 'I键', 'vk': 0x49, 'scan': 0x17},
    'J': {'name': 'J键', 'vk': 0x4A, 'scan': 0x24},
    'K': {'name': 'K键', 'vk': 0x4B, 'scan': 0x25},
    'L': {'name': 'L键', 'vk': 0x4C, 'scan': 0x26},
    'M': {'name': 'M键', 'vk': 0x4D, 'scan': 0x32},
    'N': {'name': 'N键', 'vk': 0x4E, 'scan': 0x31},
    'O': {'name': 'O键', 'vk': 0x4F, 'scan': 0x18},
    'P': {'name': 'P键', 'vk': 0x50, 'scan': 0x19},
    'Q': {'name': 'Q键', 'vk': 0x51, 'scan': 0x10},
    'R': {'name': 'R键', 'vk': 0x52, 'scan': 0x13},
    'S': {'name': 'S键', 'vk': 0x53, 'scan': 0x1F},
    'T': {'name': 'T键', 'vk': 0x54, 'scan': 0x14},
    'U': {'name': 'U键', 'vk': 0x55, 'scan': 0x16},
    'V': {'name': 'V键', 'vk': 0x56, 'scan': 0x2F},
    'W': {'name': 'W键', 'vk': 0x57, 'scan': 0x11},
    'X': {'name': 'X键', 'vk': 0x58, 'scan': 0x2D},
    'Y': {'name': 'Y键', 'vk': 0x59, 'scan': 0x15},
    'Z': {'name': 'Z键', 'vk': 0x5A, 'scan': 0x2C},
    'SPACE': {'name': '空格键', 'vk': 0x20, 'scan': 0x39},
    'ENTER': {'name': '回车键', 'vk': 0x0D, 'scan': 0x1C},
}

@dataclass
class KeyState:
    """按键状态类"""
    is_down: bool = False
    name: str = ""
    vk: int = 0
    scan: int = 0
    thread: Optional[threading.Thread] = None
    should_stop: Optional[threading.Event] = None

    def __post_init__(self):
        self.should_stop = threading.Event()

class ClickerState(Enum):
    """连发器状态枚举"""
    STOPPED = "已停止"
    RUNNING = "运行中"
    PAUSED = "已暂停"

class AutoClicker:
    def __init__(self):
        self.running = True
        self.key_states: Dict[str, KeyState] = {
            key: KeyState(
                name=info['name'],
                vk=info['vk'],
                scan=info['scan']
            ) 
            for key, info in MONITORED_KEYS.items()
        }
        self._is_target_active = False
        self._state = ClickerState.STOPPED

    def send_key_once(self, key_state: KeyState):
        """发送单次按键"""
        try:
            # 发送按键按下
            win32api.keybd_event(
                key_state.vk,          # 虚拟键码
                key_state.scan,        # 扫描码
                0,                     # 标志位
                0                      # 附加信息
            )
            time.sleep(0.001)  # 等待1ms
            # 发送按键释放
            win32api.keybd_event(
                key_state.vk,
                key_state.scan,
                win32con.KEYEVENTF_KEYUP,
                0
            )
        except Exception as e:
            print(f"发送按键时出错: {e}")

    def key_sender_thread(self, key: str):
        """单个按键的发送线程"""
        key_state = self.key_states[key]
        
        while not key_state.should_stop.is_set():
            if key_state.is_down:
                self.send_key_once(key_state)
            time.sleep(KEY_LOOP_INTERVAL)

    def get_active_window_info(self):
        """获取当前激活窗口信息"""
        try:
            hwnd = win32gui.GetForegroundWindow()
            return win32gui.GetWindowText(hwnd)
        except:
            return ""

    def check_window_state(self):
        """检查窗口状态并管理线程"""
        while self.running:
            window_title = self.get_active_window_info()
            is_target = window_title and any(
                identifier in window_title for identifier in TARGET_WINDOWS
            )
            
            if is_target != self._is_target_active:
                self._is_target_active = is_target
                if is_target:
                    print("目标窗口激活 - 启动按键线程")
                    self.start_all_key_threads()
                else:
                    print("目标窗口切换到后台 - 停止按键线程")
                    self.stop_all_key_threads()
            
            time.sleep(0.1)

    def start_all_key_threads(self):
        """启动所有按键线程"""
        for key, key_state in self.key_states.items():
            if not key_state.thread or not key_state.thread.is_alive():
                key_state.should_stop.clear()
                key_state.thread = threading.Thread(
                    target=self.key_sender_thread,
                    args=(key,),
                    name=f"{key_state.name}发送线程"
                )
                key_state.thread.daemon = True
                key_state.thread.start()

    def stop_all_key_threads(self):
        """停止所有按键线程"""
        for key_state in self.key_states.values():
            if key_state.thread and key_state.thread.is_alive():
                key_state.should_stop.set()
                key_state.thread.join(timeout=1.0)
                key_state.thread = None
            key_state.is_down = False

    def handle_key_event(self, key: str, is_down: bool):
        """处理按键事件"""
        if key not in self.key_states:
            return
            
        key_state = self.key_states[key]
        if self._is_target_active:
            if is_down and not key_state.is_down:
                print(f"{key_state.name}被按下 - 开始发送")
                key_state.is_down = True
            elif not is_down and key_state.is_down:
                print(f"{key_state.name}被释放 - 停止发送")
                key_state.is_down = False

    def setup_key_hooks(self):
        """设置按键钩子"""
        for key in self.key_states:
            keyboard.on_press_key(key, lambda e: self.handle_key_event(e.name, True))
            keyboard.on_release_key(key, lambda e: self.handle_key_event(e.name, False))

    def configure_keys(self, keys: List[str]) -> None:
        """配置需要连发的按键"""
        # 停止所有当前的按键线程
        self.stop_all_key_threads()
        
        # 只保留选中的按键
        self.key_states = {
            key: self.key_states[key.upper()]
            for key in keys
            if key.upper() in MONITORED_KEYS
        }

    def start(self):
        """启动连发器"""
        if self._state != ClickerState.STOPPED:
            return
            
        print("开始监控按键...")
        self.running = True
        self._state = ClickerState.RUNNING
        
        # 启动窗口检查线程
        window_check_thread = threading.Thread(
            target=self.check_window_state,
            name="WindowChecker"
        )
        window_check_thread.daemon = True
        window_check_thread.start()

        # 设置按键钩子
        self.setup_key_hooks()

    def stop(self):
        """停止连发器"""
        if self._state == ClickerState.STOPPED:
            return
            
        print("停止连发...")
        self.running = False
        self._state = ClickerState.STOPPED
        
        # 停止所有按键线程
        self.stop_all_key_threads()
        
        # 移除按键钩子
        keyboard.unhook_all()

    @property
    def state(self) -> ClickerState:
        """获取当前状态"""
        return self._state

    def __del__(self):
        """析构函数，确保资源清理"""
        self.stop() 