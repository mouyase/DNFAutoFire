from PyQt6.QtWidgets import QWidget, QPushButton, QGridLayout, QHBoxLayout
from PyQt6.QtGui import QFont
from typing import Dict, List

class KeyButton(QPushButton):
    def __init__(self, text: str, width: int = 50, height: int = 40):
        super().__init__(text)
        self.setFixedSize(width, height)
        self.is_active = False
        
        # 设置默认样式
        self.default_style = """
            QPushButton {
                background-color: #f0f0f0;
                border: 1px solid #ccc;
                border-radius: 4px;
            }
            QPushButton:hover {
                background-color: #e0e0e0;
            }
        """
        
        # 设置激活样式
        self.active_style = """
            QPushButton {
                background-color: #ff4444;
                color: white;
                border: 1px solid #cc0000;
                border-radius: 4px;
                font-weight: bold;
            }
            QPushButton:hover {
                background-color: #ff6666;
            }
        """
        
        self.setStyleSheet(self.default_style)
        self.clicked.connect(self.toggle_state)

    def toggle_state(self):
        self.is_active = not self.is_active
        self.update_appearance()

    def update_appearance(self):
        self.setStyleSheet(self.active_style if self.is_active else self.default_style)

    def set_active(self, active: bool):
        self.is_active = active
        self.update_appearance()

class KeyboardLayout(QWidget):
    def __init__(self):
        super().__init__()
        self.buttons: Dict[str, KeyButton] = {}
        self.initUI()

    def create_button(self, text: str, width: int = 50, height: int = 40) -> KeyButton:
        btn = KeyButton(text, width, height)
        self.buttons[text] = btn
        return btn

    def get_active_keys(self) -> List[str]:
        return [key for key, btn in self.buttons.items() if btn.is_active]

    def set_active_keys(self, keys: List[str]):
        for key, btn in self.buttons.items():
            btn.set_active(key in keys)

    def initUI(self):
        main_layout = QHBoxLayout()
        main_layout.setSpacing(20)
        
        # 左侧主键盘区域
        main_keyboard = QWidget()
        grid = QGridLayout()
        grid.setSpacing(5)
        
        # ESC 和功能键行
        esc = self.create_button("Esc")
        grid.addWidget(esc, 0, 0)
        
        # 功能键空隙
        empty_widget = QWidget()
        empty_widget.setFixedSize(20, 40)
        grid.addWidget(empty_widget, 0, 1)
        
        # F1-F12
        for i in range(12):
            btn = self.create_button(f"F{i+1}")
            grid.addWidget(btn, 0, i+2)

        # 第一行：数字键
        symbols = "`1234567890-="
        for i, char in enumerate(symbols):
            btn = self.create_button(char)
            grid.addWidget(btn, 1, i)
        backspace = self.create_button("Backspace", 100, 40)
        grid.addWidget(backspace, 1, len(symbols))

        # 第二行
        tab = self.create_button("Tab", 75, 40)
        grid.addWidget(tab, 2, 0, 1, 1)
        row1 = "QWERTYUIOP[]"
        col_offset = 1
        for char in row1:
            btn = self.create_button(char)
            grid.addWidget(btn, 2, col_offset)
            col_offset += 1
        backslash = self.create_button("\\", 75, 40)
        grid.addWidget(backslash, 2, col_offset)

        # 第三行
        caps = self.create_button("Caps Lock", 90, 40)
        grid.addWidget(caps, 3, 0, 1, 2)
        row2 = "ASDFGHJKL;'"
        col_offset = 2
        for char in row2:
            btn = self.create_button(char)
            grid.addWidget(btn, 3, col_offset)
            col_offset += 1
        enter = self.create_button("Enter", 95, 40)
        grid.addWidget(enter, 3, col_offset, 1, 2)

        # 第四行
        l_shift = self.create_button("Shift", 115, 40)
        grid.addWidget(l_shift, 4, 0, 1, 2)
        row3 = "ZXCVBNM,./"
        col_offset = 2
        for char in row3:
            btn = self.create_button(char)
            grid.addWidget(btn, 4, col_offset)
            col_offset += 1
        r_shift = self.create_button("Shift", 115, 40)
        grid.addWidget(r_shift, 4, col_offset, 1, 2)

        # 第五行
        l_ctrl = self.create_button("Ctrl", 70, 40)
        l_win = self.create_button("Win", 70, 40)
        l_alt = self.create_button("Alt", 70, 40)
        space = self.create_button("Space", 290, 40)
        r_alt = self.create_button("Alt", 70, 40)
        r_win = self.create_button("Win", 70, 40)
        r_ctrl = self.create_button("Ctrl", 70, 40)

        grid.addWidget(l_ctrl, 5, 0, 1, 1)
        grid.addWidget(l_win, 5, 1, 1, 1)
        grid.addWidget(l_alt, 5, 2, 1, 1)
        grid.addWidget(space, 5, 3, 1, 7)
        grid.addWidget(r_alt, 5, 10, 1, 1)
        grid.addWidget(r_win, 5, 11, 1, 1)
        grid.addWidget(r_ctrl, 5, 12, 1, 1)

        main_keyboard.setLayout(grid)
        main_layout.addWidget(main_keyboard)
        self.setLayout(main_layout) 