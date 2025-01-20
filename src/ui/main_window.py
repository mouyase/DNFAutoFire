from PySide6.QtWidgets import QMainWindow, QMessageBox
from ui.ui_main import Ui_MainWindow

class MainWindow(QMainWindow):
    def __init__(self):
        super().__init__()
        # 创建UI对象
        self.ui = Ui_MainWindow()
        # 设置UI
        self.ui.setupUi(self)
        # 初始化信号和槽
        self.setup_connections()
    
    def setup_connections(self):
        """设置信号和槽连接"""
        # 连接按钮的点击信号到槽函数
        # self.ui.pushButton.clicked.connect(self.on_button_clicked)
    
    def on_button_clicked(self):
        """按钮点击事件处理函数"""
        QMessageBox.information(
            self,
            "提示",
            "按钮被点击了！",
            QMessageBox.StandardButton.Ok
        )
