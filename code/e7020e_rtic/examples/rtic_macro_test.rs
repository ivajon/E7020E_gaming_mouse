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


#[rtic::app(device = stm32f4::stm32f401, dispatchers = [EXTI0])]
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

    use app::hidDescriptors::MouseKeyboard;
    use app::mouseKeyboardReport::MouseKeyboardState;
    use app::macroSystem::{MacroConfig, Function};
    
    type Button = ErasedPin<Input<PullUp>>;

    #[shared]
    struct Shared {
        mouse: MouseKeyboardState,
        macro_conf: MacroConfig
    }

    #[local]
    struct Local {
        usb_dev: UsbDevice<'static, UsbBus<USB>>,
        hid: HIDClass<'static, UsbBus<USB>>,
        button: Button
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

        let usb = USB {
            usb_global: dp.OTG_FS_GLOBAL,
            usb_device: dp.OTG_FS_DEVICE,
            usb_pwrclk: dp.OTG_FS_PWRCLK,
            pin_dm: gpioa.pa11.into_alternate(),
            pin_dp: gpioa.pa12.into_alternate(),
            hclk: clocks.hclk(),
        };

        let gpioc = dp.GPIOC.split();
        let gpiob = dp.GPIOB.split();
        let mut button = gpiob.pb1.into_pull_up_input().erase();
        let mut sys_cfg = dp.SYSCFG.constrain();

        // Enable interuppts for PC13
        button.make_interrupt_source(&mut sys_cfg);
        button.enable_interrupt(&mut dp.EXTI);
        button.trigger_on_edge(&mut dp.EXTI, Edge::RisingFalling);

        cx.local.bus.replace(UsbBus::new(usb, cx.local.EP_MEMORY));

        let hid = HIDClass::new(
            cx.local.bus.as_ref().unwrap(),
            MouseKeyboard::desc(),
            POLL_INTERVAL_MS,
        );

        let mouse = MouseKeyboardState::new();

        let mut macro_conf = MacroConfig::new();
        macro_conf.update_config(
            Function::PressKeyboard(0x4), Function::Nothing,
            Function::Nothing, Function::Nothing,
            Function::Nothing, Function::Nothing,
            Function::Nothing
        );

        let usb_dev =
            UsbDeviceBuilder::new(cx.local.bus.as_ref().unwrap(), UsbVidPid(0xc410, 0x0000))
                .manufacturer("e7020e")
                .product("Mouse")
                .serial_number("1337")
                .device_class(0) // Hid
                .build();

        (Shared {mouse, macro_conf}, Local { usb_dev, hid, button}, init::Monotonics())
    }

    // B1 is connected to PC13
    #[task(binds=EXTI1, local = [button], shared = [mouse, macro_conf])]
    fn button_pressed(mut cx: button_pressed::Context) {
        // this should be automatic
        cx.local.button.clear_interrupt_pending_bit();

        if cx.local.button.is_low() {
            rprintln!("button low");
            cx.shared.mouse.lock(|mouse| {
                cx.shared.macro_conf.lock(|macro_conf| {
                    macro_conf.push_left(mouse);
                });
            });
        } else {
            rprintln!("button high");
            cx.shared.mouse.lock(|mouse| {
                cx.shared.macro_conf.lock(|macro_conf| {
                    macro_conf.release_left(mouse);
                });
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
