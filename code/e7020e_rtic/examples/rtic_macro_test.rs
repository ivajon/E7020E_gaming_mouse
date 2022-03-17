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
    use dwt_systick_monotonic::*;
    use usb_device::{bus::UsbBusAllocator, prelude::*};
    use usbd_hid::{
        descriptor::{generator_prelude::*, MouseReport},
        hid_class::HIDClass,
    };
    use systick_monotonic::*;

    use app::hidDescriptors::MouseKeyboard;
    use app::mouseKeyboardReport::MouseKeyboardState;
    use app::macroSystem::{MacroConfig, Function, MacroType, do_function, end_function, MacroSequence};
    
    type Button = ErasedPin<Input<PullUp>>;

    // Default core clock at 16MHz
    const FREQ_CORE: u32 = 48_000_000;

    #[monotonic(binds = SysTick, default = true)]
    type MyMono = DwtSystick<FREQ_CORE>; // 16MHz cycle accurate accuracy

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

        let systick = cx.core.SYST;
        let mut dcb = cx.core.DCB;
        let dwt = cx.core.DWT;

        // Initialize the monotonic (SysTick driven by core clock)
        let mono = DwtSystick::new(&mut dcb, dwt, systick, FREQ_CORE);

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
        let mut button = gpioa.pa13.into_pull_up_input().erase();
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
            MacroType::MacroSingle(Function::PressKeyboard(0x4)),
            MacroType::MacroSingle(Function::PressKeyboard(0x4)),
            MacroType::MacroSingle(Function::Nothing),
            MacroType::MacroSingle(Function::Nothing),
            MacroType::MacroSingle(Function::Nothing),
            MacroType::MacroSingle(Function::Nothing),
            MacroType::MacroSingle(Function::Nothing),
        );

        let usb_dev =
            UsbDeviceBuilder::new(cx.local.bus.as_ref().unwrap(), UsbVidPid(0xc410, 0x0000))
                .manufacturer("e7020e")
                .product("Mouse")
                .serial_number("1337")
                .device_class(0) // Hid
                .build();

        (Shared {mouse, macro_conf}, Local { usb_dev, hid, button}, init::Monotonics(mono))
    }

    #[task(shared = [mouse, macro_conf])]
    fn do_macro(cx: do_macro::Context, m: &'static MacroSequence, i: u8) {
    }

    fn handle_macro(m: &'static MacroType, mouse: &mut MouseKeyboardState, is_push: bool) {
        match m {
            MacroType::MacroSingle(f) => {
                match is_push {
                    true => do_function(*f, mouse),
                    false => end_function(*f, mouse),
                }
            },
            MacroType::MacroMultiple(s) => {
                //let ms = s.delays[0];
                //do_macro::spawn_after(ms.millis(), s, 0).unwrap();
            }
        }
    }

    // B1 is connected to PC13
    #[task(binds=EXTI15_10, local = [button], shared = [mouse, macro_conf])]
    fn button_pressed(mut cx: button_pressed::Context) {
        // this should be automatic
        cx.local.button.clear_interrupt_pending_bit();

        if cx.local.button.is_low() {
            rprintln!("button low");
            cx.shared.macro_conf.lock(|macro_conf| {
                cx.shared.mouse.lock(|mouse| {
                    //handle_macro(&macro_conf.right_button, mouse, true);
                });
            });

        } else {
            rprintln!("button high");
            cx.shared.macro_conf.lock(|macro_conf| {
                cx.shared.mouse.lock(|mouse| {
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
