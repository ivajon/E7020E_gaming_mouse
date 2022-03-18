use hidapi::HidApi;

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

            #[repr(C, packed)]
            #[derive(Debug)]
            struct FromHost {
                id: u8,
                data: Data,
            }

            #[repr(C, packed)]
            #[derive(Debug)]
            struct Data {
                first: u32,
                second: u32,
            }

            unsafe fn any_as_u8_slice_mut<T: Sized>(p: &mut T) -> &mut [u8] {
                ::core::slice::from_raw_parts_mut(
                    (p as *mut T) as *mut u8,
                    ::core::mem::size_of::<T>(),
                )
            }

            unsafe fn any_as_u8_slice<T: Sized>(p: &T) -> &[u8] {
                ::core::slice::from_raw_parts(
                    (p as *const T) as *const u8,
                    ::core::mem::size_of::<T>(),
                )
            }

            match api.open(0xc410, 0x0000) {
                Ok(hd) => {
                    println!("open ok");

                    let mut from_host = FromHost {
                        id: 0,
                        data: Data {
                            first: 0x01234567,
                            second: 0x89abcdef,
                        },
                    };

                    let mut to_host = Data {
                        first: 0,
                        second: 0,
                    };

                    loop {
                        match hd.write(unsafe { any_as_u8_slice(&mut from_host) }) {
                            Ok(n) => {
                                println!("write: n {}", n);
                                loop {
                                    match hd.read(unsafe { any_as_u8_slice_mut(&mut to_host) }) {
                                        Ok(n) => {
                                            println!("read: n {}, data {:#08x?}", n, to_host);
                                            from_host.data.first = to_host.first;
                                            from_host.data.second = to_host.second + 1;
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
