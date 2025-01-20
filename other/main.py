import sys
from PySide6.QtWidgets import QApplication, QMainWindow
from ui.main import Ui_MainWindow
from controllers.main_controller import MainController

class MainWindow(QMainWindow):
    def __init__(self):
        super().__init__()
        # 设置UI
        self.ui = Ui_MainWindow()
        self.ui.setupUi(self)
        # 创建控制器
        self.controller = MainController(self.ui)

def main():
    app = QApplication(sys.argv)
    window = MainWindow()
    window.show()
    sys.exit(app.exec())

if __name__ == "__main__":
    main()
