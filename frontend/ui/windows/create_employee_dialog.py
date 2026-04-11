from PyQt6.QtWidgets import QDialog

from frontend.ui.py.ui_create_employee_dialog import Ui_Dialog


class CreateEmployeeDialog(QDialog, Ui_Dialog):
    def __init__(self):
        super().__init__()
        self.setupUi(self)
        self.pushButton.clicked.connect(self.accept)
        self.pushButton_2.clicked.connect(self.reject)