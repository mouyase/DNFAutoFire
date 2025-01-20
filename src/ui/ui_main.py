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
from PySide6.QtWidgets import (QApplication, QGroupBox, QMainWindow, QPushButton,
    QSizePolicy, QWidget)

class Ui_MainWindow(object):
    def setupUi(self, MainWindow):
        if not MainWindow.objectName():
            MainWindow.setObjectName(u"MainWindow")
        MainWindow.resize(940, 510)
        self.centralwidget = QWidget(MainWindow)
        self.centralwidget.setObjectName(u"centralwidget")
        self.keyboardGroup = QGroupBox(self.centralwidget)
        self.keyboardGroup.setObjectName(u"keyboardGroup")
        self.keyboardGroup.setGeometry(QRect(8, 8, 926, 276))
        self.key_esc = QPushButton(self.keyboardGroup)
        self.key_esc.setObjectName(u"key_esc")
        self.key_esc.setGeometry(QRect(8, 22, 36, 36))
        self.key_f1 = QPushButton(self.keyboardGroup)
        self.key_f1.setObjectName(u"key_f1")
        self.key_f1.setGeometry(QRect(82, 22, 36, 36))
        self.key_f2 = QPushButton(self.keyboardGroup)
        self.key_f2.setObjectName(u"key_f2")
        self.key_f2.setGeometry(QRect(122, 22, 36, 36))
        self.key_f3 = QPushButton(self.keyboardGroup)
        self.key_f3.setObjectName(u"key_f3")
        self.key_f3.setGeometry(QRect(238, 30, 36, 36))
        self.key_f4 = QPushButton(self.keyboardGroup)
        self.key_f4.setObjectName(u"key_f4")
        self.key_f4.setGeometry(QRect(312, 30, 36, 36))
        self.key_f5 = QPushButton(self.keyboardGroup)
        self.key_f5.setObjectName(u"key_f5")
        self.key_f5.setGeometry(QRect(386, 30, 36, 36))
        self.key_f6 = QPushButton(self.keyboardGroup)
        self.key_f6.setObjectName(u"key_f6")
        self.key_f6.setGeometry(QRect(460, 30, 36, 36))
        self.key_f7 = QPushButton(self.keyboardGroup)
        self.key_f7.setObjectName(u"key_f7")
        self.key_f7.setGeometry(QRect(534, 30, 36, 36))
        self.key_f8 = QPushButton(self.keyboardGroup)
        self.key_f8.setObjectName(u"key_f8")
        self.key_f8.setGeometry(QRect(608, 30, 36, 36))
        self.key_f9 = QPushButton(self.keyboardGroup)
        self.key_f9.setObjectName(u"key_f9")
        self.key_f9.setGeometry(QRect(682, 30, 36, 36))
        self.key_f10 = QPushButton(self.keyboardGroup)
        self.key_f10.setObjectName(u"key_f10")
        self.key_f10.setGeometry(QRect(756, 30, 36, 36))
        self.key_f11 = QPushButton(self.keyboardGroup)
        self.key_f11.setObjectName(u"key_f11")
        self.key_f11.setGeometry(QRect(830, 30, 36, 36))
        self.key_f12 = QPushButton(self.keyboardGroup)
        self.key_f12.setObjectName(u"key_f12")
        self.key_f12.setGeometry(QRect(904, 30, 36, 36))
        MainWindow.setCentralWidget(self.centralwidget)

        self.retranslateUi(MainWindow)

        QMetaObject.connectSlotsByName(MainWindow)
    # setupUi

    def retranslateUi(self, MainWindow):
        MainWindow.setWindowTitle(QCoreApplication.translate("MainWindow", u"DNF\u6309\u952e\u8fde\u53d1\u5de5\u5177", None))
        self.keyboardGroup.setTitle(QCoreApplication.translate("MainWindow", u"\u6309\u952e\u8bbe\u7f6e - [ \u7ea2\u8272\u4e3a\u542f\u7528\u8fde\u53d1 \u84dd\u8272\u4e3a\u5173\u95ed\u8fde\u53d1 ]", None))
        self.key_esc.setText(QCoreApplication.translate("MainWindow", u"Esc", None))
        self.key_f1.setText(QCoreApplication.translate("MainWindow", u"F1", None))
        self.key_f2.setText(QCoreApplication.translate("MainWindow", u"F2", None))
        self.key_f3.setText(QCoreApplication.translate("MainWindow", u"F3", None))
        self.key_f4.setText(QCoreApplication.translate("MainWindow", u"F4", None))
        self.key_f5.setText(QCoreApplication.translate("MainWindow", u"F5", None))
        self.key_f6.setText(QCoreApplication.translate("MainWindow", u"F6", None))
        self.key_f7.setText(QCoreApplication.translate("MainWindow", u"F7", None))
        self.key_f8.setText(QCoreApplication.translate("MainWindow", u"F8", None))
        self.key_f9.setText(QCoreApplication.translate("MainWindow", u"F9", None))
        self.key_f10.setText(QCoreApplication.translate("MainWindow", u"F10", None))
        self.key_f11.setText(QCoreApplication.translate("MainWindow", u"F11", None))
        self.key_f12.setText(QCoreApplication.translate("MainWindow", u"F12", None))
    # retranslateUi

