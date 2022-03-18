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

            match api.open(0xc410, 0x0000) {
                Ok(hd) => {
                    println!("open ok");

                    let mut buf = [0u8; 4];

                    loop {
                        hd.read(&mut buf).unwrap();
                        let u: u32 = u32::from_le_bytes(buf);
                        println!("buf {:?}, {}", buf, u);
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
