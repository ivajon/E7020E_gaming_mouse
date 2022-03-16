import usb.core
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
    dev = usb.core.find(idVendor=vendor_id, idProduct=product_id)
    if dev is None:
        raise ValueError('Device not found')
    return dev
"""
    Defines a simple usb device driver
    Inteded for use with the HID mouse and keyboard
"""
class usb_device:
    def __init__(self, vendor_id,product_id) -> None:
        self.idVendor = vendor_id
        self.idProduct = product_id
        self.dev = get_usb_device(vendor_id,product_id)
        self.interface = self.dev[0].interfaces()[0].bInterfaceNumber
        self.endpoint = self.dev[0].interfaces()[0].endpoints()[0]
        self.endpoint_adress = self.endpoint.bEndpointAddress
    def disattach(self):
        if self.dev.is_kernel_driver_active(self.interface):
            self.dev.detach_kernel_driver(self.interface)
    def attach(self):
        self.dev.attach_kernel_driver(self.interface)
    def read(self,length):
        return self.dev.read(self.endpoint_adress,length)
    def write(self,data):
        # Writes a datapacket to the device
        self.disattach()
        self.dev.write(self.endpoint_adress,data)
        self.reset()
        self.attach()
    def reset(self):
        self.dev.reset()
    def set_configuration(self):
        self.dev.set_configuration()
    def get_active_configuration(self):
        return self.dev.get_active_configuration()
    def get_active_configuration_descriptor(self):
        return self.dev.get_active_configuration_descriptor()
    def __repr__(self) -> str:
        return f"<usb_device: {self.idVendor}:{self.idProduct}>"
    def __str__(self) -> str:
        return f"<usb_device: {self.idVendor}:{self.idProduct}>"
    def __eq__(self,other) -> bool:
        return self.idVendor == other.idVendor and self.idProduct == other.idProduct





