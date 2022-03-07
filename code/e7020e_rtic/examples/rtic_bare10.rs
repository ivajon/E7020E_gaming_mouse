//! bare10.rs
//!
//! Serial
//!
//! What it covers:
//! - usb-serial communication
//! - good design
//!
//! In this case we use RTT, so change your runner accordingly in `.cargo/config`.
#![no_main]
#![no_std]

use panic_rtt_target as _;

#[rtic::app(device = stm32f4::stm32f411, dispatchers = [EXTI0])]
mod app {
    use rtt_target::{rprintln, rtt_init_print};
    use stm32f4xx_hal::otg_fs::{UsbBus, USB};
    use stm32f4xx_hal::prelude::*;
    use usb_device::{bus::UsbBusAllocator, prelude::*};
    use usbd_serial::SerialPort;

    #[shared]
    struct Shared {}

    #[local]
    struct Local {
        usb_dev: UsbDevice<'static, UsbBus<USB>>,
        serial: SerialPort<'static, UsbBus<USB>>,
    }

    #[init(local = [EP_MEMORY: [u32; 1024] = [0; 1024], bus: Option<UsbBusAllocator<UsbBus<USB>>> = None])]
    fn init(cx: init::Context) -> (Shared, Local, init::Monotonics) {
        rtt_init_print!();
        rprintln!("init");
        let dp = cx.device;

        let rcc = dp.RCC.constrain();

        let clocks = rcc.cfgr.sysclk(48.MHz()).require_pll48clk().freeze();

        let gpioa = dp.GPIOA.split();

        let usb = USB {
            usb_global: dp.OTG_FS_GLOBAL,
            usb_device: dp.OTG_FS_DEVICE,
            usb_pwrclk: dp.OTG_FS_PWRCLK,
            pin_dm: gpioa.pa11.into_alternate(),
            pin_dp: gpioa.pa12.into_alternate(),
            hclk: clocks.hclk(),
        };

        cx.local.bus.replace(UsbBus::new(usb, cx.local.EP_MEMORY));

        let serial = usbd_serial::SerialPort::new(cx.local.bus.as_ref().unwrap());

        let usb_dev =
            UsbDeviceBuilder::new(cx.local.bus.as_ref().unwrap(), UsbVidPid(0x16c0, 0x27dd))
                .manufacturer("e7020e")
                .product("Serial port")
                .serial_number("1337")
                .device_class(usbd_serial::USB_CLASS_CDC)
                .build();

        (Shared {}, Local { usb_dev, serial }, init::Monotonics())
    }

    #[task(binds=OTG_FS, local = [usb_dev, serial])]
    fn usb_fs(cx: usb_fs::Context) {
        let serial = cx.local.serial;
        let usb_dev = cx.local.usb_dev;

        if usb_dev.poll(&mut [serial]) {
            let mut buf = [0u8; 64];

            match serial.read(&mut buf) {
                Ok(count) if count > 0 => {
                    // Echo back in upper case
                    for c in buf[0..count].iter_mut() {
                        if 0x61 <= *c && *c <= 0x7a {
                            *c &= !0x20;
                        }
                    }

                    let mut write_offset = 0;
                    while write_offset < count {
                        match serial.write(&buf[write_offset..count]) {
                            Ok(len) if len > 0 => {
                                write_offset += len;
                            }
                            _ => {}
                        }
                    }
                }
                _ => {}
            }
        }
    }
}

// 0. Preparation:
//
//    Cut a usb cable and connect it to the nucleo (pa11/pa12/gnd).
//
//    Hint:
//    Check the USB standard for color markings, and the Nucleo data sheet
//    for the header layout.
//
//    Run the example in release mode.
//
//    Check that the usb device is visible
//    > lsusb
//    Bus 003 Device 013: ID 16c0:27dd Van Ooijen Technische Informatica CDC-ACM class devices (modems)
//
//    Now you should be able to connect to the Virtual Com Port using
//    `cutecom`. Typically /dev/tty/ACM1 as ACM0 is the Nucleo VCP.
//
//    Verify that it it works (echoing the data).
//
// 1. Compare the two solutions in `rtic_usb_serial` to the code in this file.
//
//    In particular make sure you have understood how the EP_MEMORY is handled, and
//    the `usb_dev` and `usb_serial` is moved to the `usb_fs` task.
//
//    Explain in you own words where `usb_dev` and `usb_fs` are
//    stored in the two solutions.
//
//    ** your answer here **
//
// 2. Take inspiration from your `rtic_bare9` code and re-implement
//    error handling and logging in a similar fashion.
//
//    Commit you code as `bare10_2`.
//
// 3. Discussion:
//
//    Now we have a standalone USB serial VCP.
//    This can ideally deliver a 12MBit connection (Full Speed).
//    https://en.wikipedia.org/wiki/USB
//
//    A very useful and fast data channel useful to many
//    embedded applications, easy to setup and use.
