use hidapi::HidApi;
use std::ops::{Deref, DerefMut};

fn main() {
    println!("Printing all available hid devices:");

    match HidApi::new() {
        Ok(api) => {
            for device in api.device_list() {
                println!(
                    "VID {:04x}: PID {:04x}",
                    device.vendor_id(),
                    device.product_id()
                );
                println!("manufacturer {:?}", device.manufacturer_string());
                println!("product {:?}", device.product_string());
                println!("serial {:?}", device.serial_number());
            }

            // Open Iris

            #[repr(C)]
            #[derive(Debug, Copy, Clone)]
            struct Data {
                ticks: u32,
            }

            #[repr(C)]
            #[derive(Copy, Clone)]
            union HidArray<T>
            where
                T: Sized,
            {
                data: std::mem::ManuallyDrop<T>,
                array: [u8; 65], // max size of hid packed + 1
            }

            impl<T> HidArray<T>
            where
                T: Sized,
            {
                fn get_array(&mut self) -> &mut [u8] {
                    unsafe { &mut self.array[..std::mem::size_of::<T>()] }
                }

                fn new(data: T) -> Self {
                    Self {
                        data: std::mem::ManuallyDrop::new(data),
                    }
                }
            }

            impl<T> Deref for HidArray<T>
            where
                T: Sized,
            {
                type Target = T;

                fn deref(&self) -> &T {
                    unsafe { &self.data }
                }
            }

            impl<T> DerefMut for HidArray<T>
            where
                T: Sized,
            {
                fn deref_mut(&mut self) -> &mut T {
                    unsafe { &mut self.data }
                }
            }

            #[repr(C, packed)]
            struct To<T> {
                id: u8,
                data: T,
            }

            impl<T> To<T> {
                fn set_data(&mut self, data: T) {
                    let ptr = std::ptr::addr_of_mut!(self.data);
                    unsafe { std::ptr::write_unaligned(ptr, data) };
                }

                fn get_data(&self) -> T {
                    let ptr = std::ptr::addr_of!(self.data);
                    unsafe { std::ptr::read_unaligned(ptr) }
                }
            }

            let mut from_host = HidArray::new(To {
                id: 0,
                data: Data { ticks: 0 },
            });

            let mut to_host = HidArray::new(Data { ticks: 0 });

            match api.open(0xc410, 0x0000) {
                Ok(hd) => {
                    println!("open ok");

                    loop {
                        match hd.write(from_host.get_array()) {
                            Ok(n) => {
                                println!("write: n {}", n);
                                loop {
                                    match hd.read(&mut to_host.get_array()) {
                                        Ok(n) => {
                                            println!("read: n {}, data {:#08x?}", n, *to_host);

                                            from_host.set_data(*to_host);

                                            break;
                                        }
                                        Err(err) => {
                                            println!("err {}", err);
                                        }
                                    };
                                }
                            }
                            Err(err) => {
                                println!("err {}", err);
                            }
                        }
                    }
                }
                Err(err) => println!("error {:?}", err),
            }
        }
        Err(e) => {
            eprintln!("Error: {}", e);
        }
    }
}
