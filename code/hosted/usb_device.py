import hid
import numpy as np

"""
    Helper functions, not really part of the API
"""
def to_byte(data : int,min_size :int = 2):
        ret = []
        while data:
            ret =[data & 0xFF] + ret
            data >>= 8
        while len(ret) < min_size:
            ret = [0] + ret
        return ret
def get_usb_device(vendor_id = 0, product_id = 0):
    h = hid.device()
    h.open(vendor_id,product_id)
    print("Manufacturer: %s" % h.get_manufacturer_string())
    print("Product: %s" % h.get_product_string())
    print("Serial No: %s" % h.get_serial_number_string())
    # enable non-blocking mode
    h.set_nonblocking(1)
    return h
"""
    Defines a simple usb device driver
    Inteded for use with the HID mouse and keyboard
"""
class usb_device:
    def __init__(self, vendor_id,product_id) -> None:
        self.idVendor = vendor_id
        self.idProduct = product_id
        self.dev = get_usb_device(vendor_id,product_id)
    def read(self,length):
        return self.dev.read()
    def write(self,data):
        self.dev.write(data)
    def __repr__(self) -> str:
        return f"<usb_device: {self.idVendor}:{self.idProduct}>"
    def __str__(self) -> str:
        return f"<usb_device: {self.idVendor}:{self.idProduct}>"





