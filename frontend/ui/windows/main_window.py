from PyQt6.QtWidgets import QMainWindow, QHeaderView, QTableWidgetItem
from httpx import Client

from frontend.ui.py.ui_main_window import Ui_MainWindow


class MainWindow(QMainWindow, Ui_MainWindow):
    def __init__(self, client: Client):
        super().__init__()
        self.setupUi(self)
        self.client = client

        self.setup_table()

    def setup_table(self):
        table = self.tableWidget
        table.setColumnCount(6)
        table.setHorizontalHeaderLabels(['id', 'name', 'surname', 'patronymic', 'salary', 'works_since'])

        header = table.horizontalHeader()
        header.setSectionResizeMode(QHeaderView.ResizeMode.ResizeToContents)

        self.update()
        self.pushButton.clicked.connect(self.update)

    def update(self):
        table = self.tableWidget
        data = self.client.get('/employee').json()
        table.setRowCount(len(data))
        for row, items in enumerate(data):
            for column, value in enumerate(items.items()):
                (key, item) = value
                if key == 'salary':
                    item = f"{item['amount']} {item['currency']}"
                table.setItem(row, column, QTableWidgetItem(item))
