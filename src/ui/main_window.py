from typing import Dict, Set
from PySide6.QtWidgets import QMainWindow, QPushButton, QSystemTrayIcon, QMenu
from PySide6.QtCore import Qt
from PySide6.QtGui import QIcon, QCloseEvent

from .ui_main import Ui_MainWindow
from ..core.auto_fire import AutoFire
from ..core.key_codes import KeyCodes
from ..utils.config import Config


class MainWindow(QMainWindow):
    def __init__(self):
        super().__init__()
        # 初始化UI
        self.ui = Ui_MainWindow()
        self.ui.setupUi(self)
        
        # 初始化配置和自动连发
        self.config = Config()
        self.enabled_keys: Set[int] = self.config.load_enabled_keys()
        self.auto_fire = AutoFire(self.enabled_keys)
        
        # 初始化按键映射
        self.key_buttons: Dict[str, QPushButton] = {}
        self.scan_code_map: Dict[str, int] = {}
        self._init_key_buttons()
        
        # 初始化托盘图标
        self.tray_icon = self._init_tray_icon()
        
        # 设置窗口标志
        self.setWindowFlags(
            Qt.WindowType.WindowStaysOnTopHint |  # 窗口置顶
            Qt.WindowType.WindowMinimizeButtonHint |  # 最小化按钮
            Qt.WindowType.WindowCloseButtonHint  # 关闭按钮
        )

    def _init_key_buttons(self) -> None:
        """初始化所有按键按钮"""
        # 获取所有以"key_"开头的按钮
        for attr_name in dir(self.ui):
            if attr_name.startswith('key_'):
                button = getattr(self.ui, attr_name)
                if isinstance(button, QPushButton):
                    # 保存按钮引用
                    key_name = attr_name[4:]  # 移除"key_"前缀
                    self.key_buttons[key_name] = button
                    
                    # 获取扫描码
                    scan_code = KeyCodes.get_scan_code(key_name)
                    if scan_code:
                        self.scan_code_map[key_name] = scan_code
                        
                        # 设置按钮状态
                        if scan_code in self.enabled_keys:
                            self._set_button_enabled(button, True)
                        
                        # 连接点击事件
                        button.clicked.connect(
                            lambda checked, btn=button, name=key_name: 
                            self._on_key_button_clicked(btn, name)
                        )

    def _init_tray_icon(self) -> QSystemTrayIcon:
        """初始化托盘图标"""
        tray_icon = QSystemTrayIcon(self)
        tray_icon.setIcon(QIcon("path/to/icon.png"))  # TODO: 添加图标
        
        # 创建托盘菜单
        tray_menu = QMenu()
        show_action = tray_menu.addAction("显示")
        show_action.triggered.connect(self.show)
        quit_action = tray_menu.addAction("退出")
        quit_action.triggered.connect(self.close)
        
        tray_icon.setContextMenu(tray_menu)
        tray_icon.show()
        
        return tray_icon

    def _set_button_enabled(self, button: QPushButton, enabled: bool) -> None:
        """设置按钮状态"""
        if enabled:
            button.setStyleSheet("background-color: #ffaaaa;")  # 红色表示启用
        else:
            button.setStyleSheet("background-color: #aaaaff;")  # 蓝色表示禁用

    def _on_key_button_clicked(self, button: QPushButton, key_name: str) -> None:
        """处理按键按钮点击事件"""
        scan_code = self.scan_code_map.get(key_name)
        if not scan_code:
            return
            
        # 切换状态
        is_enabled = scan_code in self.enabled_keys
        if is_enabled:
            self.enabled_keys.remove(scan_code)
        else:
            self.enabled_keys.add(scan_code)
            
        # 更新按钮状态
        self._set_button_enabled(button, not is_enabled)
        
        # 保存配置
        self.config.save_enabled_keys(self.enabled_keys)
        
        # 更新AutoFire
        self.auto_fire.update_key_codes(self.enabled_keys)

    def closeEvent(self, event: QCloseEvent) -> None:
        """处理窗口关闭事件"""
        if self.tray_icon.isVisible():
            self.hide()
            event.ignore()
        else:
            self.auto_fire.stop()
            event.accept()
