# -*- coding: utf-8 -*-

################################################################################
## Form generated from reading UI file 'main.ui'
##
## Created by: Qt User Interface Compiler version 6.8.1
##
## WARNING! All changes made in this file will be lost when recompiling UI file!
################################################################################

from PySide6.QtCore import (QCoreApplication, QDate, QDateTime, QLocale,
    QMetaObject, QObject, QPoint, QRect,
    QSize, QTime, QUrl, Qt)
from PySide6.QtGui import (QBrush, QColor, QConicalGradient, QCursor,
    QFont, QFontDatabase, QGradient, QIcon,
    QImage, QKeySequence, QLinearGradient, QPainter,
    QPalette, QPixmap, QRadialGradient, QTransform)
from PySide6.QtWidgets import (QApplication, QGridLayout, QMainWindow, QPushButton,
    QSizePolicy, QSpacerItem, QVBoxLayout, QWidget)

class Ui_MainWindow(object):
    def setupUi(self, MainWindow):
        if not MainWindow.objectName():
            MainWindow.setObjectName(u"MainWindow")
        MainWindow.resize(1300, 500)
        MainWindow.setMinimumSize(QSize(1300, 500))
        MainWindow.setMaximumSize(QSize(1300, 500))
        self.centralwidget = QWidget(MainWindow)
        self.centralwidget.setObjectName(u"centralwidget")
        self.verticalLayout = QVBoxLayout(self.centralwidget)
        self.verticalLayout.setSpacing(0)
        self.verticalLayout.setObjectName(u"verticalLayout")
        self.verticalLayout.setContentsMargins(0, 0, 0, 0)
        self.keyboard_widget = QWidget(self.centralwidget)
        self.keyboard_widget.setObjectName(u"keyboard_widget")
        self.keyboard_layout = QGridLayout(self.keyboard_widget)
        self.keyboard_layout.setSpacing(0)
        self.keyboard_layout.setObjectName(u"keyboard_layout")
        self.keyboard_layout.setContentsMargins(0, 0, 0, 0)
        self.key_esc = QPushButton(self.keyboard_widget)
        self.key_esc.setObjectName(u"key_esc")
        sizePolicy = QSizePolicy(QSizePolicy.Policy.Fixed, QSizePolicy.Policy.Fixed)
        sizePolicy.setHorizontalStretch(0)
        sizePolicy.setVerticalStretch(0)
        sizePolicy.setHeightForWidth(self.key_esc.sizePolicy().hasHeightForWidth())
        self.key_esc.setSizePolicy(sizePolicy)
        self.key_esc.setMinimumSize(QSize(40, 40))
        self.key_esc.setMaximumSize(QSize(40, 40))

        self.keyboard_layout.addWidget(self.key_esc, 0, 0, 1, 1)

        self.horizontalSpacer = QSpacerItem(5, 20, QSizePolicy.Policy.Fixed, QSizePolicy.Policy.Minimum)

        self.keyboard_layout.addItem(self.horizontalSpacer, 0, 1, 1, 1)

        self.key_f1 = QPushButton(self.keyboard_widget)
        self.key_f1.setObjectName(u"key_f1")
        sizePolicy.setHeightForWidth(self.key_f1.sizePolicy().hasHeightForWidth())
        self.key_f1.setSizePolicy(sizePolicy)
        self.key_f1.setMinimumSize(QSize(40, 40))
        self.key_f1.setMaximumSize(QSize(40, 40))

        self.keyboard_layout.addWidget(self.key_f1, 0, 2, 1, 1)

        self.key_f2 = QPushButton(self.keyboard_widget)
        self.key_f2.setObjectName(u"key_f2")
        sizePolicy.setHeightForWidth(self.key_f2.sizePolicy().hasHeightForWidth())
        self.key_f2.setSizePolicy(sizePolicy)
        self.key_f2.setMinimumSize(QSize(40, 40))
        self.key_f2.setMaximumSize(QSize(40, 40))

        self.keyboard_layout.addWidget(self.key_f2, 0, 3, 1, 1)

        self.key_f3 = QPushButton(self.keyboard_widget)
        self.key_f3.setObjectName(u"key_f3")
        sizePolicy.setHeightForWidth(self.key_f3.sizePolicy().hasHeightForWidth())
        self.key_f3.setSizePolicy(sizePolicy)
        self.key_f3.setMinimumSize(QSize(40, 40))
        self.key_f3.setMaximumSize(QSize(40, 40))

        self.keyboard_layout.addWidget(self.key_f3, 0, 4, 1, 1)

        self.key_f4 = QPushButton(self.keyboard_widget)
        self.key_f4.setObjectName(u"key_f4")
        sizePolicy.setHeightForWidth(self.key_f4.sizePolicy().hasHeightForWidth())
        self.key_f4.setSizePolicy(sizePolicy)
        self.key_f4.setMinimumSize(QSize(40, 40))
        self.key_f4.setMaximumSize(QSize(40, 40))

        self.keyboard_layout.addWidget(self.key_f4, 0, 5, 1, 1)


        self.verticalLayout.addWidget(self.keyboard_widget)

        MainWindow.setCentralWidget(self.centralwidget)

        self.retranslateUi(MainWindow)

        QMetaObject.connectSlotsByName(MainWindow)
    # setupUi

    def retranslateUi(self, MainWindow):
        MainWindow.setWindowTitle(QCoreApplication.translate("MainWindow", u"DNF\u6309\u952e\u8fde\u53d1\u5de5\u5177", None))
        self.key_esc.setText(QCoreApplication.translate("MainWindow", u"Esc", None))
        self.key_f1.setText(QCoreApplication.translate("MainWindow", u"F1", None))
        self.key_f2.setText(QCoreApplication.translate("MainWindow", u"F2", None))
        self.key_f3.setText(QCoreApplication.translate("MainWindow", u"F3", None))
        self.key_f4.setText(QCoreApplication.translate("MainWindow", u"F4", None))
    # retranslateUi

