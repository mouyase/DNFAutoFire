import keyboard
import win32gui
import win32api
import win32con
import threading
import time
from dataclasses import dataclass
from typing import Dict

# 常量定义
DNF_WINDOW_IDENTIFIERS = ["地下城与勇士", "DNF"]
KEY_LOOP_INTERVAL = 0.001  # 1ms的循环间隔

# 定义需要连发的按键及其虚拟键码
MONITORED_KEYS = {
    'j': {'name': 'J键', 'vk': 0xFF, 'scan': 0x24},  # 36
    'p': {'name': 'P键', 'vk': 0xFF, 'scan': 0x19},  # 25
    'l': {'name': 'L键', 'vk': 0xFF, 'scan': 0x26},  # 38
    'h': {'name': 'H键', 'vk': 0xFF, 'scan': 0x23}   # 35
}

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
        self.key_states: Dict[str, KeyState] = {
            key: KeyState(
                name=info['name'],
                vk=info['vk'],
                scan=info['scan']
            ) 
            for key, info in MONITORED_KEYS.items()
        }
        self._is_dnf_active = False

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
        """获取当前激活窗口的信息"""
        try:
            window_title = win32gui.GetWindowText(win32gui.GetForegroundWindow())
            return window_title
        except:
            return None

    def check_window_state(self):
        """检查DNF窗口状态并管理线程"""
        while self.running:
            window_title = self.get_active_window_info()
            is_dnf = window_title and any(
                identifier in window_title for identifier in DNF_WINDOW_IDENTIFIERS
            )
            
            if is_dnf != self._is_dnf_active:
                self._is_dnf_active = is_dnf
                if is_dnf:
                    print("DNF窗口激活 - 启动按键线程")
                    self.start_all_key_threads()
                else:
                    print("DNF窗口切换到后台 - 停止按键线程")
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
        if self._is_dnf_active:
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
        """启动程序"""
        print("开始监控按键，按ESC退出...")
        
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
        self.cleanup(window_check_thread)

    def cleanup(self, window_check_thread):
        """清理资源"""
        self.running = False
        window_check_thread.join(timeout=1.0)
        self.stop_all_key_threads()

def main():
    clicker = VirtualKeyClicker()
    clicker.start()

if __name__ == "__main__":
    main() 