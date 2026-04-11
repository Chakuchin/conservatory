import time

from PyQt6.QtWidgets import QMainWindow, QHeaderView, QTableWidgetItem, QDialog, QMessageBox
from httpx import Client

from frontend.ui.py.ui_main_window import Ui_MainWindow
from frontend.ui.windows.create_employee_dialog import CreateEmployeeDialog

def flatten_to_dict(d):
    result = {}
    stack = [d]
    while stack:
        current = stack.pop()
        for k, v in current.items():
            if isinstance(v, dict):
                stack.append(v)
            else:
                result[k] = v
    return result

class MainWindow(QMainWindow, Ui_MainWindow):
    def __init__(self, client: Client):
        super().__init__()
        self.setupUi(self)
        self.client = client

        self.pushButton_2.clicked.connect(self.create_employee)
        self.setup_employee_table()
        self.setup_greenhouse_table()
        self.setup_plant_table()

    def setup_employee_table(self):
        table = self.tableWidget
        table.setColumnCount(6)
        table.setHorizontalHeaderLabels(['id', 'name', 'surname', 'patronymic', 'salary', 'works_since'])

        header = table.horizontalHeader()
        header.setSectionResizeMode(QHeaderView.ResizeMode.ResizeToContents)

        self.update_employee_table()
        self.pushButton.clicked.connect(self.update_employee_table)

    def setup_greenhouse_table(self):
        table = self.tableWidget_2
        table.setColumnCount(6)
        table.setHorizontalHeaderLabels(['id', 'name', 'humidity', 'target_temperature', 'area', 'conditions'])

        header = table.horizontalHeader()
        header.setSectionResizeMode(QHeaderView.ResizeMode.ResizeToContents)

        self.update_greenhouse_table()
        self.pushButton.clicked.connect(self.update_greenhouse_table)

    def setup_plant_table(self):
        table = self.tableWidget_3
        table.setColumnCount(5)
        table.setHorizontalHeaderLabels(['id', 'planted_at', 'type_urn', 'name', 'description'])

        header = table.horizontalHeader()
        header.setSectionResizeMode(QHeaderView.ResizeMode.ResizeToContents)

        self.update_plant_table()
        self.pushButton.clicked.connect(self.update_plant_table)

    def update_employee_table(self):
        table = self.tableWidget
        data = self.client.get('/employee').json()
        table.setRowCount(len(data))
        for row, items in enumerate(data):

            for column, value in enumerate(items.items()):
                (key, item) = value
                if key == 'salary':
                    item = f"{item['amount']} {item['currency']}"
                table.setItem(row, column, QTableWidgetItem(item))

    def update_greenhouse_table(self):
        table = self.tableWidget_2
        data = self.client.get('/greenhouse').json()
        table.setRowCount(len(data))
        for row, items in enumerate(data):
            for column, value in enumerate(items.items()):
                (key, item) = value
                if key == 'conditions':
                    item = ', '.join(item).strip()
                table.setItem(row, column, QTableWidgetItem(str(item)))

    def update_plant_table(self):
        table = self.tableWidget_3
        data = self.client.get('/plant').json()
        table.setRowCount(len(data))
        for row, items in enumerate(data):
            items = flatten_to_dict(items)
            for column, value in enumerate(items.items()):
                (key, item) = value
                table.setItem(row, column, QTableWidgetItem(item))

    def create_employee(self):
        dialog = CreateEmployeeDialog()
        if dialog.exec() == QDialog.DialogCode.Accepted:
            if dialog.radioButton.isChecked():
                currency = dialog.radioButton.text()
            elif dialog.radioButton_2.isChecked():
                currency = dialog.radioButton_2.text()
            else:
                QMessageBox.warning(self, "Warning", "Can't add employee with empty input")
                return
            if dialog.lineEdit.text().strip() == '' or dialog.lineEdit.text().strip() == ''\
                    or dialog.lineEdit_3.text().strip() == '' or dialog.spinBox.value() <= 0:
                QMessageBox.warning(self, "Warning", "Can't add employee with empty input")
                return
            self.client.post('/employee', json={
                'name': dialog.lineEdit.text().strip(),
                'surname': dialog.lineEdit_2.text().strip(),
                'patronymic': dialog.lineEdit_3.text().strip(),
                'salary': {
                    'amount': dialog.spinBox.value(),
                    'currency': currency
                },
                'works_since': time.strftime('%Y-%m-%d', time.localtime(time.time()))
            })
            self.update_employee_table()
