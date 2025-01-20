from PyQt6.QtWidgets import (QWidget, QPushButton, QHBoxLayout, QVBoxLayout, 
                            QListWidget, QLineEdit, QMessageBox)
from typing import Callable, List

class ConfigManager(QWidget):
    def __init__(self, on_load: Callable, on_save: Callable, on_delete: Callable,
                 on_start: Callable):
        super().__init__()
        self.on_load = on_load
        self.on_save = on_save
        self.on_delete = on_delete
        self.on_start = on_start
        self.initUI()

    def initUI(self):
        layout = QHBoxLayout()
        
        # 配置列表
        self.config_list = QListWidget()
        layout.addWidget(self.config_list)
        
        # 控制区域
        control_layout = QVBoxLayout()
        
        # 配置名输入框
        self.config_name_input = QLineEdit()
        self.config_name_input.setPlaceholderText("输入配置名称")
        control_layout.addWidget(self.config_name_input)
        
        # 按钮区域
        button_layout = QHBoxLayout()
        
        load_btn = QPushButton("读取配置")
        save_btn = QPushButton("保存配置")
        delete_btn = QPushButton("删除配置")
        
        load_btn.clicked.connect(self.load_config)
        save_btn.clicked.connect(self.save_config)
        delete_btn.clicked.connect(self.delete_config)
        
        button_layout.addWidget(load_btn)
        button_layout.addWidget(save_btn)
        button_layout.addWidget(delete_btn)
        
        control_layout.addLayout(button_layout)
        
        # 启动按钮
        start_btn = QPushButton("启动连发")
        start_btn.clicked.connect(self.on_start)
        control_layout.addWidget(start_btn)
        
        control_widget = QWidget()
        control_widget.setLayout(control_layout)
        layout.addWidget(control_widget)
        
        self.setLayout(layout)

    def load_config(self):
        selected_items = self.config_list.selectedItems()
        if not selected_items:
            QMessageBox.warning(self, "警告", "请先选择一个配置")
            return
        self.on_load(selected_items[0].text())

    def save_config(self):
        config_name = self.config_name_input.text().strip()
        if not config_name:
            QMessageBox.warning(self, "警告", "请输入配置名称")
            return
        self.on_save(config_name)

    def delete_config(self):
        selected_items = self.config_list.selectedItems()
        if not selected_items:
            QMessageBox.warning(self, "警告", "请先选择一个配置")
            return
        
        config_name = selected_items[0].text()
        reply = QMessageBox.question(self, "确认", f"确定要删除配置 {config_name} 吗？",
                                   QMessageBox.StandardButton.Yes | QMessageBox.StandardButton.No)
        
        if reply == QMessageBox.StandardButton.Yes:
            self.on_delete(config_name)

    def update_config_list(self, config_names: List[str]):
        self.config_list.clear()
        self.config_list.addItems(config_names)

    def get_config_name(self) -> str:
        return self.config_name_input.text().strip()

    def set_config_name(self, name: str):
        self.config_name_input.setText(name) 