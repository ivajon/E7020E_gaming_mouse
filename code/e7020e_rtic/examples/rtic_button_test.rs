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

#[rtic::app(device = stm32f4::stm32f401, dispatchers = [DMA1_STREAM0,DMA1_STREAM1])]
mod app {
    // Relative app imports
    // Absolute imports
    use rtt_target::{rprintln, rtt_init_print};
    use stm32f4xx_hal::otg_fs::{UsbBus, USB};
    use stm32f4xx_hal::prelude::*;
    use stm32f4xx_hal::gpio::*;
    use dwt_systick_monotonic::*;
    use usb_device::{bus::UsbBusAllocator};


    // Default core clock at 16MHz
    const FREQ_CORE: u32 = 16_000_000;

    #[monotonic(binds = SysTick, default = true)]
    type MyMono = DwtSystick<FREQ_CORE>; // 16MHz cycle accurate accuracy

    //type Button = ErasedPin<Input<PullUp>>;
    type Button = ErasedPin<Input<PullDown>>;
    
    #[shared]
    struct Shared {
    }
    
    #[local]
    struct Local {
        button: Button,
    }

    #[init(local = [EP_MEMORY: [u32; 1024] = [0; 1024], bus: Option<UsbBusAllocator<UsbBus<USB>>> = None])]
    fn init(cx: init::Context) -> (Shared, Local, init::Monotonics) {
        rtt_init_print!();
        rprintln!("init");

        // grab core and device pointers
        let mut cd = cx.core;
        let mut dp = cx.device;

        // Systic config
        let mut sys_cfg = dp.SYSCFG.constrain();
        let mono = DwtSystick::new(&mut cd.DCB, cd.DWT,cd.SYST , FREQ_CORE);

        // Grab gpio pins
        let gpioa = dp.GPIOA.split();
        let gpiob = dp.GPIOB.split();
        let gpioc = dp.GPIOC.split();

        // Configure IO pins
        let mut button = gpioc.pc12.into_pull_down_input().erase();
        // Enable button button interrupt
        button.make_interrupt_source(&mut sys_cfg);
        button.enable_interrupt(&mut dp.EXTI);
        button.trigger_on_edge(&mut dp.EXTI, Edge::RisingFalling);

        (
            Shared {},
            Local { button},
            init::Monotonics(mono)
        )
    }
    #[task(binds=EXTI15_10, local = [button], shared = [])]
    fn button_hand(mut cx: button_hand::Context) {
        // this should be automatic
        cx.local.button.clear_interrupt_pending_bit();

        if cx.local.button.is_low() {
            rprintln!("button low");
        } else {
            rprintln!("button high");
        }
    }

        
    
    
    
}
