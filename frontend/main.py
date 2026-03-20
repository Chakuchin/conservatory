import httpx
import sys

from PyQt6.QtWidgets import QApplication

from frontend.ui.windows.main_window import MainWindow


def main():
    app = QApplication(sys.argv)
    with httpx.Client(base_url="http://localhost:8080/api/v1") as client:
        w = MainWindow(client)
        w.show()
        status = app.exec()
    sys.exit(status)


if __name__=="__main__":
    main()