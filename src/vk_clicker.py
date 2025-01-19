import keyboard
import win32gui
import win32api
import win32con
import threading
import time
from dataclasses import dataclass
from typing import Dict

# 常量定义
TARGET_WINDOWS = ["地下城与勇士", "DNF"]
KEY_LOOP_INTERVAL = 0.001  # 1ms的循环间隔

@dataclass
class KeyState:
    """按键状态类"""
    is_down: bool = False
    name: str = ""
    vk: int = 0
    scan: int = 0
    thread: threading.Thread = None
    should_stop: threading.Event = None

    def __post_init__(self):
        self.should_stop = threading.Event()

class VirtualKeyClicker:
    def __init__(self):
        self.running = True
        self.key_states: Dict[str, KeyState] = {}
        self._is_target_active = False

    def configure_keys(self, keys_config: Dict[str, Dict]):
        """配置需要连发的按键
        例如: {
            'j': {'name': 'J键', 'vk': 0x4A, 'scan': 0x24},
            'k': {'name': 'K键', 'vk': 0x4B, 'scan': 0x25}
        }
        """
        self.key_states = {
            key: KeyState(
                name=info['name'],
                vk=info['vk'],
                scan=info['scan']
            ) 
            for key, info in keys_config.items()
        }

    def send_key_once(self, key_state: KeyState):
        """发送单次按键"""
        try:
            win32api.keybd_event(key_state.vk, key_state.scan, 0, 0)
            time.sleep(0.001)
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
        """获取当前激活窗口的信息"""
        try:
            return win32gui.GetWindowText(win32gui.GetForegroundWindow())
        except:
            return None

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

    def start(self):
        """启动连发器"""
        print("开始监控按键...")
        self.running = True
        
        # 启动窗口检查线程
        window_check_thread = threading.Thread(
            target=self.check_window_state,
            name="WindowChecker"
        )
        window_check_thread.daemon = True
        window_check_thread.start()

        # 设置按键钩子
        self.setup_key_hooks()

        # 等待ESC退出
        keyboard.wait('esc')
        print("检测到ESC，退出程序...")
        
        # 清理资源
        self.running = False
        self.stop_all_key_threads()
        keyboard.unhook_all()
        window_check_thread.join(timeout=1.0)

if __name__ == "__main__":
    # 创建连发器实例
    clicker = VirtualKeyClicker()
    
    # 配置J键连发
    keys_config = {
        'j': {'name': 'J键', 'vk': 0x4A, 'scan': 0x24}
    }
    clicker.configure_keys(keys_config)
    
    # 启动连发器
    clicker.start()