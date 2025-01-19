from PyQt6.QtWidgets import QApplication, QMainWindow, QWidget, QVBoxLayout, QMessageBox
from PyQt6.QtCore import Qt
import sys
from value import APP_NAME
from config import Config
from keyboard_ui import KeyboardLayout
from config_ui import ConfigManager
from auto_clicker import AutoClicker, ClickerState

class MainWindow(QMainWindow):
    def __init__(self):
        super().__init__()
        self.config = Config()
        self.auto_clicker = AutoClicker()
        self.keyboard = None
        self.config_manager = None
        self.initUI()

    def initUI(self):
        self.setWindowTitle(APP_NAME)
        self.setGeometry(100, 100, 1300, 500)
        self.setFixedSize(1300, 500)
        self.setWindowFlags(Qt.WindowType.WindowMinimizeButtonHint | Qt.WindowType.WindowCloseButtonHint)
        
        # 创建主布局
        main_widget = QWidget()
        main_layout = QVBoxLayout()
        
        # 添加键盘布局
        self.keyboard = KeyboardLayout()
        main_layout.addWidget(self.keyboard)
        
        # 添加配置管理器
        self.config_manager = ConfigManager(
            on_load=self.load_config,
            on_save=self.save_config,
            on_delete=self.delete_config,
            on_start=self.toggle_auto_fire
        )
        # 初始化配置列表
        self.config_manager.update_config_list(self.config.get_config_names())
        main_layout.addWidget(self.config_manager)
        
        main_widget.setLayout(main_layout)
        self.setCentralWidget(main_widget)

    def load_config(self, config_name: str):
        if self.config.load_config(config_name):
            self.keyboard.set_active_keys(self.config.current_config['enabled_keys'])
            self.config_manager.set_config_name(config_name)
            QMessageBox.information(self, "成功", f"已加载配置：{config_name}")
        else:
            QMessageBox.warning(self, "错误", "加载配置失败")

    def save_config(self, config_name: str):
        active_keys = self.keyboard.get_active_keys()
        self.config.current_config['enabled_keys'] = active_keys
        self.config.save_current_config(config_name)
        self.config_manager.update_config_list(self.config.get_config_names())
        QMessageBox.information(self, "成功", f"配置已保存：{config_name}")

    def delete_config(self, config_name: str):
        if self.config.delete_config(config_name):
            self.config_manager.update_config_list(self.config.get_config_names())
            QMessageBox.information(self, "成功", "配置已删除")
        else:
            QMessageBox.warning(self, "错误", "删除配置失败")

    def toggle_auto_fire(self):
        active_keys = self.keyboard.get_active_keys()
        if not active_keys:
            QMessageBox.warning(self, "警告", "请先选择要连发的按键")
            return
        
        if self.auto_clicker.state == ClickerState.STOPPED:
            self.auto_clicker.configure_keys(active_keys)
            self.auto_clicker.start()
            QMessageBox.information(self, "成功", f"已启动连发：{', '.join(active_keys)}")
        else:
            self.auto_clicker.stop()
            QMessageBox.information(self, "成功", "已停止连发")

def main():
    app = QApplication(sys.argv)
    window = MainWindow()
    window.show()
    sys.exit(app.exec())

if __name__ == "__main__":
    main()