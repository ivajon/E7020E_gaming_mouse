//! rtic_usb_mouse.rs
//!
//! Mouse HID example
//!
//! What it covers:
//! - Mouse HID communication
//! - good design, interrupt driven polling
//!
//! In this case we use RTT, so change your runner accordingly in `.cargo/config`.
#![no_main]
#![no_std]

use panic_rtt_target as _;

#[rtic::app(device = stm32f4::stm32f401, dispatchers = [DMA1_STREAM0])]
mod app {
    use rtt_target::{rprintln, rtt_init_print};
    use stm32f4xx_hal::otg_fs::{UsbBus, USB};
    use stm32f4xx_hal::prelude::*;
    use stm32f4xx_hal::gpio::*;
    use usb_device::{bus::UsbBusAllocator, prelude::*};
    use usbd_hid::{
        descriptor::{generator_prelude::*, MouseReport},
        hid_class::HIDClass,
    };

    use app::mouseReport::MouseState;

    //type Button = ErasedPin<Input<PullUp>>;
    type Button = ErasedPin<Input<PullDown>>;

    #[shared]
    struct Shared {
        mouse: MouseState
    }

    #[local]
    struct Local {
        usb_dev: UsbDevice<'static, UsbBus<USB>>,
        hid: HIDClass<'static, UsbBus<USB>>,
        left: Button,
        right: Button,
        middle: Button,
        front: Button,
        back: Button,
    }

    const POLL_INTERVAL_MS: u8 = 1;

    #[init(local = [EP_MEMORY: [u32; 1024] = [0; 1024], bus: Option<UsbBusAllocator<UsbBus<USB>>> = None])]
    fn init(cx: init::Context) -> (Shared, Local, init::Monotonics) {
        rtt_init_print!();
        rprintln!("init");
        let mut dp = cx.device;
        let cd = cx.core;

        let rcc = dp.RCC.constrain();

        let clocks = rcc.cfgr.sysclk(48.MHz()).require_pll48clk().freeze();

        let gpioa = dp.GPIOA.split();
        let gpiob = dp.GPIOB.split();
        let gpioc = dp.GPIOC.split();

        let usb = USB {
            usb_global: dp.OTG_FS_GLOBAL,
            usb_device: dp.OTG_FS_DEVICE,
            usb_pwrclk: dp.OTG_FS_PWRCLK,
            pin_dm: gpioa.pa11.into_alternate(),
            pin_dp: gpioa.pa12.into_alternate(),
            hclk: clocks.hclk(),
        };

        let mut left = gpiob.pb0.into_pull_down_input().erase();
        let mut right = gpiob.pb1.into_pull_down_input().erase();
        let mut middle = gpiob.pb12.into_pull_down_input().erase();
        let mut front = gpioc.pc5.into_pull_down_input().erase();
        let mut back = gpioc.pc4.into_pull_down_input().erase();


        /*
        loop {
            if left.is_high() {
                rprintln!("is high");
            } else {
                rprintln!("is low");
            }
        }
        */

        let mut sys_cfg = dp.SYSCFG.constrain();

        // Enable interuppts for the buttons
        left.make_interrupt_source(&mut sys_cfg);
        left.enable_interrupt(&mut dp.EXTI);
        left.trigger_on_edge(&mut dp.EXTI, Edge::RisingFalling);

        right.make_interrupt_source(&mut sys_cfg);
        right.enable_interrupt(&mut dp.EXTI);
        right.trigger_on_edge(&mut dp.EXTI, Edge::RisingFalling);

        middle.make_interrupt_source(&mut sys_cfg);
        middle.enable_interrupt(&mut dp.EXTI);
        middle.trigger_on_edge(&mut dp.EXTI, Edge::RisingFalling);

        front.make_interrupt_source(&mut sys_cfg);
        front.enable_interrupt(&mut dp.EXTI);
        front.trigger_on_edge(&mut dp.EXTI, Edge::RisingFalling);

        back.make_interrupt_source(&mut sys_cfg);
        back.enable_interrupt(&mut dp.EXTI);
        back.trigger_on_edge(&mut dp.EXTI, Edge::RisingFalling);

        cx.local.bus.replace(UsbBus::new(usb, cx.local.EP_MEMORY));

        let hid = HIDClass::new(
            cx.local.bus.as_ref().unwrap(),
            MouseReport::desc(),
            POLL_INTERVAL_MS,
        );

        let mouse = MouseState::new();

        let usb_dev =
            UsbDeviceBuilder::new(cx.local.bus.as_ref().unwrap(), UsbVidPid(0xc410, 0x0000))
                .manufacturer("e7020e")
                .product("Mouse")
                .serial_number("1337")
                .device_class(0) // Hid
                .build();

        (
            Shared {mouse},
            Local { usb_dev, hid, left, right, middle, front, back},
            init::Monotonics()
        )
    }

    #[task(binds=EXTI15_10, local = [middle], shared = [mouse])]
    fn middle_hand(mut cx: middle_hand::Context) {
        // this should be automatic
        cx.local.middle.clear_interrupt_pending_bit();

        if cx.local.middle.is_low() {
            rprintln!("middle low");
            cx.shared.mouse.lock(|mouse| {
                mouse.push_middle();
            });
        } else {
            rprintln!("middle high");
            cx.shared.mouse.lock(|mouse| {
                mouse.release_middle();
            });
        }
    }

    #[task(binds=EXTI0, local = [left], shared = [mouse])]
    fn left_hand(mut cx: left_hand::Context) {
        // this should be automatic
        cx.local.left.clear_interrupt_pending_bit();

        if cx.local.left.is_low() {
            rprintln!("left low");
            cx.shared.mouse.lock(|mouse| {
                mouse.push_left();
            });
        } else {
            rprintln!("left high");
            cx.shared.mouse.lock(|mouse| {
                mouse.release_left();
            });
        }
    }

    #[task(binds=EXTI1, local = [right], shared = [mouse])]
    fn right_hand(mut cx: right_hand::Context) {
        // this should be automatic
        cx.local.right.clear_interrupt_pending_bit();

        if cx.local.right.is_low() {
            rprintln!("right low");
            cx.shared.mouse.lock(|mouse| {
                mouse.push_right();
            });
        } else {
            rprintln!("right high");
            cx.shared.mouse.lock(|mouse| {
                mouse.release_right();
            });
        }
    }

    #[task(binds=EXTI9_5, local = [front], shared = [mouse])]
    fn front_hand(mut cx: front_hand::Context) {
        // this should be automatic
        cx.local.front.clear_interrupt_pending_bit();

        if cx.local.front.is_low() {
            rprintln!("front low");
            cx.shared.mouse.lock(|mouse| {
                //mouse.push_front();
            });
        } else {
            rprintln!("front high");
            cx.shared.mouse.lock(|mouse| {
                //mouse.release_front();
            });
        }
    }

    #[task(binds=EXTI4, local = [back], shared = [mouse])]
    fn back_hand(mut cx: back_hand::Context) {
        // this should be automatic
        cx.local.back.clear_interrupt_pending_bit();

        if cx.local.back.is_low() {
            rprintln!("back low");
            cx.shared.mouse.lock(|mouse| {
                //mouse.push_front();
            });
        } else {
            rprintln!("back high");
            cx.shared.mouse.lock(|mouse| {
                //mouse.release_front();
            });
        }
    }

    // interrupt generated each time the hid device is polled
    // in this example each 1ms (POLL_INTERVAL_MS = 1)
    #[task(
        binds=OTG_FS,
        local = [usb_dev, hid, first :bool = true, counter:u16 = 0],
        shared = [mouse]
    )]
    fn usb_fs(mut cx: usb_fs::Context) {
        let usb_fs::LocalResources {
            usb_dev,
            hid,
            first,
            counter,
        } = cx.local;

        if *first {
            rprintln!("first");
            *first = false;
        }

        // wraps around after 200ms
        *counter = (*counter + 1) % 200;
        let mov = match *counter {
            // reached after 100ms
            100 => {
                rprintln!("10");
                10
            }
            // reached after 200ms
            0 => {
                rprintln!("-10");
                -10
            }
            _ => 0,
        };

        cx.shared.mouse.lock(|mouse| {
            mouse.add_x_movement(mov);
        });

        cx.shared.mouse.lock(|mouse| {
            let report = mouse.get_report_and_reset();
            // push the report
            hid.push_input(&report).ok();
        });


        // update the usb device state
        if usb_dev.poll(&mut [hid]) {
            return;
        }
    }
}
