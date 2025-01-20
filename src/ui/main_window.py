from typing import Dict, Set
from PySide6.QtWidgets import QMainWindow, QPushButton, QSystemTrayIcon, QMenu, QMessageBox
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
        
        # 初始化配置
        self.config = Config()
        self.enabled_keys: Set[int] = self.config.load_enabled_keys()
        self.auto_fire = None  # 先不创建AutoFire实例
        
        # 初始化按键映射
        self.key_buttons: Dict[str, QPushButton] = {}
        self.scan_code_map: Dict[str, int] = {}
        self._init_key_buttons()
        
        # 初始化启动按钮
        self.ui.start_button.clicked.connect(self._on_start_button_clicked)
        
        # 初始化配置管理
        self._init_config_management()
        
        # 设置窗口标志
        self.setWindowFlags(
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

    def _set_button_enabled(self, button: QPushButton, enabled: bool) -> None:
        """设置按钮状态"""
        if enabled:
            button.setStyleSheet("color: #ff0000;")  # 红色文本表示启用
        else:
            button.setStyleSheet("")  # 恢复默认文本颜色

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

    def _on_start_button_clicked(self) -> None:
        """处理启动按钮点击事件"""
        if self.auto_fire:
            self.auto_fire.stop()
            self.auto_fire = None
            self.ui.start_button.setText("启动")
        else:
            self.auto_fire = AutoFire(key_codes=self.enabled_keys)
            self.auto_fire.start()
            self.ui.start_button.setText("停止")

    def closeEvent(self, event: QCloseEvent) -> None:
        """处理窗口关闭事件"""
        if self.auto_fire:
            self.auto_fire.stop()
        event.accept()

    def _init_config_management(self):
        """初始化配置管理相关的控件和事件"""
        # 连接按钮事件
        self.ui.load_config.clicked.connect(self._on_load_config)
        self.ui.save_config.clicked.connect(self._on_save_config)
        self.ui.delete_config.clicked.connect(self._on_delete_config)
        
        # 加载配置列表
        self._refresh_config_list()

    def _refresh_config_list(self):
        """刷新配置列表"""
        self.ui.config_list.clear()
        configs = self.config.get_config_list()  # 假设Config类有这个方法
        self.ui.config_list.addItems(configs)

    def _on_config_selected(self, config_name: str):
        """处理配置选择事件"""
        if config_name:
            self.ui.config_name.setText(config_name)

    def _on_load_config(self):
        """处理读取配置事件"""
        # 获取当前选中的配置名
        current_item = self.ui.config_list.currentItem()
        if not current_item:
            return
            
        config_name = current_item.text()
        if not config_name:
            return
            
        # 更新输入框
        self.ui.config_name.setText(config_name)
            
        # 如果正在运行，先停止
        if self.auto_fire:
            self.auto_fire.stop()
            self.auto_fire = None
            self.ui.start_button.setText("启动")
            
        # 加载配置
        keys = self.config.load_config(config_name)
        if keys is None:
            return
            
        # 更新按键状态
        self.enabled_keys = keys
        
        # 更新所有按钮的显示状态
        for key_name, button in self.key_buttons.items():
            scan_code = self.scan_code_map.get(key_name)
            if scan_code:
                self._set_button_enabled(button, scan_code in self.enabled_keys)

    def _on_save_config(self):
        """处理保存配置事件"""
        config_name = self.ui.config_name.text().strip()
        if not config_name:
            QMessageBox.warning(self, "警告", "请输入配置名称")
            return
            
        # 保存配置
        self.config.save_config(config_name, self.enabled_keys)  # 假设Config类有这个方法
        
        # 刷新配置列表
        self._refresh_config_list()

    def _on_delete_config(self):
        """处理删除配置事件"""
        config_name = self.ui.config_name.text().strip()
        if not config_name:
            return
            
        # 确认删除
        reply = QMessageBox.question(
            self,
            "确认删除",
            f"确定要删除配置 '{config_name}' 吗？",
            QMessageBox.Yes | QMessageBox.No,
            QMessageBox.No
        )
        
        if reply == QMessageBox.Yes:
            # 删除配置
            self.config.delete_config(config_name)  # 假设Config类有这个方法
            # 清空输入框
            self.ui.config_name.clear()
            # 刷新配置列表
            self._refresh_config_list()
