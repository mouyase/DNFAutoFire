from dataclasses import dataclass
import threading
import time
from typing import Dict, Set

import keyboard
import win32api
import win32con

from ..utils.is_dnf_active import is_dnf_active
from src.core.key_codes import KeyCodes


@dataclass
class KeyState:
    """按键状态类，用于追踪每个按键的状态和相关线程"""
    is_down: bool = False
    scan_code: int = 0
    thread: threading.Thread | None = None
    should_stop: threading.Event | None = None

    def __post_init__(self) -> None:
        self.should_stop = threading.Event()


class AutoFire:
    """自动连发核心类"""
    
    def __init__(self, key_codes: Set[int], key_loop_interval: float = 0.001) -> None:
        """
        初始化连发器
        Args:
            key_codes: 按键扫描码集合
            key_loop_interval: 按键发送间隔
        """
        self.running: bool = False
        self.key_loop_interval: float = key_loop_interval
        self._is_dnf_active: bool = False
        self._window_check_thread: threading.Thread | None = None
        
        # 初始化按键状态字典
        self.key_states: Dict[str, KeyState] = {
            KeyCodes.get_key_name(code).lower(): KeyState(scan_code=code)
            for code in key_codes
        }

    @staticmethod
    def _send_virtual_key(scan_code: int) -> None:
        """
        发送虚拟按键事件
        Args:
            scan_code: 按键扫描码
        """
        try:
            win32api.keybd_event(0xFF, scan_code)  # 发送按键按下
            time.sleep(0.001)
            win32api.keybd_event(0xFF, scan_code, win32con.KEYEVENTF_KEYUP)  # 发送按键释放
        except Exception as e:
            print(f"发送按键时出错: {e}")

    # 按键发送相关方法
    def send_key_once(self, key_state: KeyState) -> None:
        """发送单次按键"""
        self._send_virtual_key(key_state.scan_code)

    def key_sender_thread(self, key: str) -> None:
        """单个按键的发送线程"""
        key_state = self.key_states[key]
        while not key_state.should_stop.is_set():
            if key_state.is_down:
                self.send_key_once(key_state)
            time.sleep(self.key_loop_interval)

    # 线程管理相关方法
    def start_all_key_threads(self) -> None:
        """启动所有按键线程"""
        for key, key_state in self.key_states.items():
            if not key_state.thread or not key_state.thread.is_alive():
                key_state.should_stop.clear()
                key_state.thread = threading.Thread(
                    target=self.key_sender_thread,
                    args=(key,),
                    name=f"{key}发送线程"
                )
                key_state.thread.daemon = True
                key_state.thread.start()

    def stop_all_key_threads(self) -> None:
        """停止所有按键线程"""
        for key_state in self.key_states.values():
            if key_state.thread and key_state.thread.is_alive():
                key_state.should_stop.set()
                key_state.thread.join(timeout=1.0)
                key_state.thread = None
            key_state.is_down = False

    # 窗口状态检查相关方法
    def check_window_state(self) -> None:
        """检查DNF窗口状态并管理线程"""
        while self.running:
            is_dnf = is_dnf_active()
            if is_dnf != self._is_dnf_active:
                self._is_dnf_active = is_dnf
                if is_dnf:
                    print("DNF窗口激活 - 启动按键线程")
                    self.start_all_key_threads()
                else:
                    print("DNF窗口切换到后台 - 停止按键线程")
                    self.stop_all_key_threads()
            time.sleep(0.1)

    # 按键事件处理相关方法
    def handle_key_event(self, key: str, is_down: bool) -> None:
        """处理按键事件"""
        if key not in self.key_states:
            return
            
        key_state = self.key_states[key]
        if self._is_dnf_active:
            if is_down and not key_state.is_down:
                print(f"{key}键被按下 - 开始发送")
                key_state.is_down = True
            elif not is_down and key_state.is_down:
                print(f"{key}键被释放 - 停止发送")
                key_state.is_down = False

    def setup_key_hooks(self) -> None:
        """设置按键钩子"""
        for key in self.key_states:
            keyboard.on_press_key(key, lambda e: self.handle_key_event(e.name, True))
            keyboard.on_release_key(key, lambda e: self.handle_key_event(e.name, False))

    # 程序控制相关方法
    def start(self) -> None:
        """启动连发程序"""
        if self.running:
            return
            
        self.running = True
        print("开始监控按键...")
        
        self._window_check_thread = threading.Thread(
            target=self.check_window_state,
            name="WindowChecker"
        )
        self._window_check_thread.daemon = True
        self._window_check_thread.start()
        
        self.setup_key_hooks()

    def stop(self) -> None:
        """停止连发程序"""
        if not self.running:
            return
            
        print("停止连发程序...")
        self.cleanup(self._window_check_thread)
        keyboard.unhook_all()

    def cleanup(self, window_check_thread: threading.Thread | None) -> None:
        """清理资源"""
        self.running = False
        if window_check_thread and window_check_thread.is_alive():
            window_check_thread.join(timeout=1.0)
        self.stop_all_key_threads()


def main() -> None:
    """测试用主函数"""
    test_config = {KeyCodes.J, KeyCodes.K, KeyCodes.L, KeyCodes.H}
    auto_fire = AutoFire(test_config)
    auto_fire.start()
    
    print("按ESC键退出测试...")
    keyboard.wait('esc')
    print("检测到ESC，退出程序...")
    auto_fire.stop()


if __name__ == "__main__":
    main() 