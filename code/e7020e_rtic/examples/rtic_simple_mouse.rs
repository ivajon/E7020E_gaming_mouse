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
    use app::pmw3389::Pmw3389;
    use rtt_target::{rprintln, rtt_init_print};
    use stm32f4xx_hal::otg_fs::{UsbBus, USB};
    use stm32f4xx_hal::prelude::*;
    use stm32f4xx_hal::gpio::*;
    use embedded_hal::spi::MODE_3;
    use usb_device::{bus::UsbBusAllocator, prelude::*};
    use usbd_hid::{
        descriptor::{generator_prelude::*, MouseReport},
        hid_class::HIDClass,
    };
    use stm32f4xx_hal::{
        gpio::{Alternate, Output, Pin, PushPull, Speed},
        prelude::*,
        spi::{Spi, TransferModeNormal},
        timer::Delay,
    };

    use stm32f4::stm32f401::{SPI1, TIM5};
    use app::mouseReport::MouseState;

    //type Button = ErasedPin<Input<PullUp>>;
    type Button = ErasedPin<Input<PullDown>>;
    // types need to be concrete for storage in a resource
    type SCK = Pin<Alternate<PushPull, 5_u8>, 'A', 5_u8>;
    type MOSI = Pin<Alternate<PushPull, 5_u8>, 'A', 7_u8>;
    type MISO = Pin<Alternate<PushPull, 5_u8>, 'A', 6_u8>;
    type CS = Pin<Output<PushPull>, 'A', 4_u8>;

    type SPI = Spi<SPI1, (SCK, MISO, MOSI), TransferModeNormal>;
    type DELAY = Delay<TIM5, 1000000_u32>;
    type PMW3389 = Pmw3389<SPI, CS, DELAY>;
    #[shared]
    struct Shared {
        mouse: MouseState,
        pmw3389: PMW3389

    }
    #[repr(u8)]
    enum API{
        RGB_CONTOLL = 0x01,
        dpi_CONTROLL = 0x02,
        DPI_CONTROLL = 0x03,
        MACRO_CONTOLL = 0x04,
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
        phase_a : Button,
        phase_b : Button,
        motion : Button
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


        let sck: SCK = gpioa.pa5.into_alternate().set_speed(Speed::VeryHigh);
        let miso: MISO = gpioa.pa6.into_alternate().set_speed(Speed::High);
        let mosi: MOSI = gpioa.pa7.into_alternate().set_speed(Speed::High);
        let cs: CS = gpioa.pa4.into_push_pull_output().set_speed(Speed::High);
        let spi: SPI = Spi::new(dp.SPI1, (sck, miso, mosi), MODE_3, 1.MHz(), &clocks);
        let delay: DELAY = dp.TIM5.delay_us(&clocks);
        let mut pmw3389: PMW3389 = Pmw3389::new(spi, cs, delay).unwrap();
        let mut motion = gpiob.pb13.into_pull_down_input().erase();
        let mut phase_a = gpiob.pb2.into_pull_down_input().erase();
        let mut phase_b = gpioc.pc9.into_pull_down_input().erase();
        let mut left = gpiob.pb0.into_pull_down_input().erase();
        let mut right = gpiob.pb1.into_pull_down_input().erase();
        let mut middle = gpiob.pb12.into_pull_down_input().erase();
        let mut front = gpioc.pc5.into_pull_down_input().erase();
        let mut back = gpioc.pc4.into_pull_down_input().erase();

        // Write the cpi regs on startup
        pmw3389.store_cpi().ok();
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
        // enable scroll wheel
        phase_a.make_interrupt_source(&mut sys_cfg);
        phase_b.make_interrupt_source(&mut sys_cfg);
        phase_a.enable_interrupt(&mut dp.EXTI);
        phase_b.enable_interrupt(&mut dp.EXTI);
        phase_a.trigger_on_edge(&mut dp.EXTI, Edge::Rising);
        phase_b.trigger_on_edge(&mut dp.EXTI, Edge::Rising);
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
                .manufacturer("Ivar och Erik")
                .product("Banger gaming mus")
                .serial_number("1234")
                .device_class(0) // Hid
                .build();

        (
            Shared {mouse,pmw3389},
            Local { usb_dev, hid, left, right, middle, front, back, phase_a,phase_b,motion},
            init::Monotonics()
        )
    }
    #[task(binds=EXTI15_10, local = [middle,motion], shared = [mouse])]
    fn middle_hand(mut cx: middle_hand::Context) {
        // this should be automatic
        cx.local.middle.clear_interrupt_pending_bit();
        
        if cx.local.middle.is_low() {
                rprintln!("middle low");
                cx.shared.mouse.lock(|mouse| {
                    mouse.release_middle();
                });
            } else if cx.local.middle.is_high() {
                rprintln!("middle high");
                cx.shared.mouse.lock(|mouse| {
                    mouse.push_middle();
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
                mouse.release_left();
            });
        } else {
            rprintln!("left high");
            cx.shared.mouse.lock(|mouse| {
                mouse.push_left();
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
                mouse.release_right();
            });
        } else {
            rprintln!("right high");
            cx.shared.mouse.lock(|mouse| {
                mouse.push_right();
            });
        }
    }

    #[task(binds=EXTI9_5, local = [front], shared = [mouse,pmw3389])]
    fn front_hand(mut cx: front_hand::Context) {
        // this should be automatic
        cx.local.front.clear_interrupt_pending_bit();
        if cx.local.front.is_low() {
            cx.shared.mouse.lock(|mouse| {
                
            });
        } else {
            rprintln!("front high");
            cx.shared.mouse.lock(|mouse| {
                cx.shared.pmw3389.lock(|pmw3389| {
                    pmw3389.increment_dpi(1);
                });
                //mouse.push_front();
            });
        }
    }

    #[task(binds=EXTI4, local = [back], shared = [mouse,pmw3389])]
    fn back_hand(mut cx: back_hand::Context) {
        
        // this should be automatic
        cx.local.back.clear_interrupt_pending_bit();
        if cx.local.back.is_low() {
            rprintln!("back low");
            cx.shared.mouse.lock(|mouse| {
                cx.shared.pmw3389.lock(|pmw3389| {
                    pmw3389.increment_dpi(-1);
                });
                //mouse.release_front();
            });
        } else {
            //rprintln!("back high");
            cx.shared.mouse.lock(|mouse| {
                
                //mouse.push_front();
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

        // Buffer could be extended if needed
        let mut buf = [0u8; 16];
        match hid.pull_raw_output(&mut buf).ok(){
            // Should return almost istantaneously if there is no data
            Some(len) => {
                // The mouse has been polled for update purposes
                handle_host_call::spawn(buf).unwrap();
            },
            None => {
            }
        }
        // The mouse has been polled for non update purposes
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
    #[task()]
    fn handle_host_call(mut cx :handle_host_call::Context,buffer : [u8;16]) {
        rprintln!("handle host call");
        rprintln!("{:?}", buffer);
        // Defines an api
        match buffer[0]{
            0x01 => {
                rprintln!("RGB _controll");
            },
            0x02 => {
                rprintln!("DPI _controll");
                // In this case the next 2 bytes are the new dpi
                let dpi = (buffer[1] as u16) << 8 | buffer[2] as u16;
                handle_dpi::spawn(dpi).unwrap();
            },
            0x03 => {
                rprintln!("DPI _controll");
            },
            0x04 => {
                rprintln!("Macro _controll");
            },
            _ => {
                rprintln!("unknown");
            }
        }

    }
    #[task(shared = [pmw3389])]
    fn handle_dpi(mut cx : handle_dpi::Context,dpi : u16){
        // Catches error when sending 2,32,32 which occurs on reset
        if dpi ==  8224{
            return;
        }

        //rprintln!("New DPI : {:}", dpi);
        //rprintln!("handle dpi");
        cx.shared.pmw3389.lock(|pmw3389| {
            pmw3389.set_dpi(dpi);
        });
    }
    #[idle(shared = [mouse,pmw3389])]
    fn idle(mut cx: idle::Context) -> ! {
        // This is just an example that measures the drift in a test rig
        // it polls the sensor at 10Hz, you can do it much quicker as well.
        //
        // Internally it has a much hight sample frequency of the camera images
        // and it buffers/accumulates the deltas between polls.
        //
        // Reading the motion value will latch the current state into the
        // DeltaXL,XH, etc., while at the same time reset the accumulators.
        //
        // This way, the polling and measurements done by the PMW3389 will be
        // completely asynchronous. You have to make sure that you poll
        // often enough for accumulated deltas not to saturate the 16 bit.
        // This could happen if you have set a high dpi, move mouse fast
        // and poll at a too low rate.
        //
        // In your case you will poll at 1ms, so I think its is physically
        // impossible to drag the mouse fast enough to saturate.
        //
        // There might be some flag set if saturated but you could
        // Check if any measurement exceeds 10% of i16::MAX, this
        // should give you immense headroom, when setting the dpi.
        //
        loop {
            let status   = cx.shared.pmw3389.lock(|pmw3389| {
                pmw3389.read_status()
            });
            match status{
                Ok(status) => {
                    if status.dx!=0 || status.dy!=0
                    {
                        cx.shared.mouse.lock(|mouse| {
                            mouse.add_x_movement(status.dx as i8 );
                            mouse.add_y_movement(status.dy as i8);
                        });   
                    }
                },
                Err(e) => {
                    rprintln!("{:?}",e);
                }
            } 
            
            
        }
    }
    
    
}
