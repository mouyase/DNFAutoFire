import interception
from interception.constants import FilterKeyFlag, KeyFlag
import time
import threading
import queue
import win32gui
import win32process
from dataclasses import dataclass
from typing import Dict
import concurrent.futures

# 常量定义
SCAN_CODE_ESC = 1
SCAN_CODE_J = 36
SCAN_CODE_P = 25
SCAN_CODE_L = 38
SCAN_CODE_H = 35  # H键的扫描码

DNF_WINDOW_IDENTIFIERS = ["地下城与勇士", "DNF"]

# 按键时间设置
KEY_PRESS_TIME = 0.001    # 1ms从按下到释放的总时间
KEY_LOOP_INTERVAL = 0.001 # 1ms的循环间隔，性能拉满

@dataclass
class KeyState:
    """按键状态类"""
    is_down: bool = False
    name: str = ""
    thread: threading.Thread = None
    should_stop: threading.Event = None

    def __post_init__(self):
        self.should_stop = threading.Event()

class KeyboardInterceptor:
    def __init__(self):
        self.context = interception.Interception()
        self.running = True
        self.key_states: Dict[int, KeyState] = {
            SCAN_CODE_J: KeyState(name="J键"),
            SCAN_CODE_P: KeyState(name="P键"),
            SCAN_CODE_L: KeyState(name="L键"),
            SCAN_CODE_H: KeyState(name="H键")  # 添加H键
        }
        self._is_dnf_active = False
        
        # 预先创建所有按键的stroke对象
        self.key_strokes = {
            code: (
                interception.KeyStroke(code, KeyFlag.KEY_DOWN),
                interception.KeyStroke(code, KeyFlag.KEY_UP)
            )
            for code in self.key_states.keys()
        }

    def get_active_window_info(self):
        """获取当前激活窗口的信息"""
        try:
            hwnd = win32gui.GetForegroundWindow()
            window_title = win32gui.GetWindowText(hwnd)
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
            
            # DNF窗口状态发生变化
            if is_dnf != self._is_dnf_active:
                self._is_dnf_active = is_dnf
                if is_dnf:
                    print("DNF窗口激活 - 启动按键线程")
                    self.start_all_key_threads()
                else:
                    print("DNF窗口切换到后台 - 停止按键线程")
                    self.stop_all_key_threads()
            
            time.sleep(0.1)  # 每100ms检查一次窗口状态

    def send_key_once(self, scan_code: int):
        """发送单次按键"""
        # 使用预创建的stroke对象
        down_stroke, up_stroke = self.key_strokes[scan_code]
        self.context.send(1, down_stroke)
        time.sleep(KEY_PRESS_TIME)
        self.context.send(1, up_stroke)

    def key_sender_thread(self, scan_code: int):
        """单个按键的发送线程"""
        key_state = self.key_states[scan_code]
        
        while not key_state.should_stop.is_set():
            if key_state.is_down:
                try:
                    self.send_key_once(scan_code)
                except Exception as e:
                    print(f"发送按键时出错: {e}")
            time.sleep(KEY_LOOP_INTERVAL)

    def start_all_key_threads(self):
        """启动所有按键线程"""
        for scan_code, key_state in self.key_states.items():
            if not key_state.thread or not key_state.thread.is_alive():
                key_state.should_stop.clear()
                key_state.thread = threading.Thread(
                    target=self.key_sender_thread,
                    args=(scan_code,),
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
            key_state.is_down = False  # 重置按键状态

    def handle_monitored_key(self, device, stroke):
        """处理被监控的按键事件"""
        if stroke.code not in self.key_states:
            return False
            
        key_state = self.key_states[stroke.code]
        if self._is_dnf_active:
            # 只更新按键状态，不启动/停止线程
            if stroke.flags == KeyFlag.KEY_DOWN:
                if not key_state.is_down:
                    print(f"{key_state.name}被按下 - 开始发送")
                    key_state.is_down = True
            elif stroke.flags == KeyFlag.KEY_UP:
                if key_state.is_down:
                    print(f"{key_state.name}被释放 - 停止发送")
                    key_state.is_down = False
        else:
            # 非DNF窗口时直接传递按键事件
            self.context.send(device, stroke)
        return True

    def start_interception(self):
        """开始键盘拦截"""
        self.context.set_filter(
            self.context.is_keyboard,
            FilterKeyFlag.FILTER_KEY_DOWN | FilterKeyFlag.FILTER_KEY_UP
        )

        # 启动窗口状态检查线程
        window_check_thread = threading.Thread(
            target=self.check_window_state,
            name="WindowChecker"
        )
        window_check_thread.daemon = True
        window_check_thread.start()

        print("开始拦截按键，按ESC退出...")
        
        try:
            self.main_loop()
        finally:
            self.cleanup(window_check_thread)

    def main_loop(self):
        """主循环处理键盘事件"""
        while True:
            device = self.context.await_input()
            if device is None:
                continue
                
            stroke = self.context.devices[device].receive()
            if stroke is None:
                continue

            if stroke.code == SCAN_CODE_ESC and stroke.flags == KeyFlag.KEY_DOWN:
                print("检测到ESC，退出拦截...")
                break
                
            if not self.handle_monitored_key(device, stroke):
                self.context.send(device, stroke)

    def cleanup(self, window_check_thread):
        """清理资源"""
        self.running = False
        window_check_thread.join(timeout=1.0)
        self.stop_all_key_threads()
        self.context.destroy()

def main():
    interception.auto_capture_devices(keyboard=True, mouse=False)
    interceptor = KeyboardInterceptor()
    interceptor.start_interception()

if __name__ == "__main__":
    main()