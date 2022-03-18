use hidapi::HidApi;
use std::ops::{Deref, DerefMut};

use serde_derive::{Deserialize, Serialize};
use ssmarshal::{deserialize, serialize};

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

            #[derive(Debug, Copy, Clone, PartialEq, Eq, Serialize, Deserialize)]
            struct Data {
                ticks: u32,
            }

            let mut buf = [0u8; 65]; // size of max HID + 1;

            let mut data = Data { ticks: 0 };
            let size = core::mem::size_of::<Data>();

            match api.open(0xc410, 0x0000) {
                Ok(hd) => {
                    println!("open ok");

                    loop {
                        // For some reason the HidApi requires a report ID
                        // Set it to 0, if no report IDs are used
                        buf[0] = 0;
                        // Serialize the the data starting from 1
                        serialize(&mut buf[1..], &data);
                        // Send the buffer (note ..=size to include the added 0 element)
                        match hd.write(&mut buf[0..=size]) {
                            Ok(n) => {
                                println!("write: n {}", n);
                                loop {
                                    // read the corresponding report
                                    match hd.read(&mut buf) {
                                        Ok(n) => {
                                            // n is the number of bytes read in the report
                                            println!("read: n = {}, raw {:x?}", n, &buf[0..n]);

                                            // deserialize the buffer
                                            data = deserialize(&buf).unwrap().0;
                                            println!("read: {:?}", data);

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
