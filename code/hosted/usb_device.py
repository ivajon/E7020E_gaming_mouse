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



"""
    Defines a simple usb device driver
    Inteded for use with the HID mouse and keyboard
"""
class usb_device:
    def __init__(self, vendor_id,product_id) -> None:
        self.idVendor = vendor_id
        self.idProduct = product_id
        self.dev = hid.device()
        #self.open()
    def open(self):
        self.dev.open(self.idVendor,self.idProduct)
        #self.dev.set_nonblocking(1)
    def close(self):
        self.dev.close()
    def read(self,length):
        return self.dev.read()
    def write(self,data):
        self.open()
        self.dev.write([0]+data+[0] * 32)
        self.close()
    def __repr__(self) -> str:
        return f"<usb_device: {self.idVendor}:{self.idProduct}>"
    def __str__(self) -> str:
        return f"<usb_device: {self.idVendor}:{self.idProduct}>"





