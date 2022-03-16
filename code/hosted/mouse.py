
from mouse_api import api
from usb_device import *


class Mouse(usb_device):
    def __init__(self, vendor_id, product_id) -> None:
        super().__init__(vendor_id, product_id)
        self.api = api()
        # Takes an int and returns a list of bytes
    def set_dpi(self, dpi : int):
        x = int(dpi)
        self.write([self.api.DPI_CONTROLL]+to_byte(x)[-2:])
