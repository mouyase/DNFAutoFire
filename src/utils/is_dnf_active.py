import win32gui
import win32process
import win32api

def is_dnf_active() -> bool:
    """判断DNF窗口是否被激活
    Returns:
        bool: 如果DNF窗口被激活返回True，否则返回False
    """
    try:
        # 获取当前激活窗口的进程ID
        hwnd = win32gui.GetForegroundWindow()
        _, pid = win32process.GetWindowThreadProcessId(hwnd)
        
        # 获取进程句柄
        handle = win32api.OpenProcess(0x0400 | 0x0010, False, pid)
        
        # 获取进程名
        exe_path = win32process.GetModuleFileNameEx(handle, 0)
        exe_name = exe_path.split('\\')[-1].lower()
        
        return exe_name == "dnf.exe"
    except:
        return False
