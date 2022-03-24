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
//type Button = ErasedPin<Input<PullUp>>;
#[rtic::app(device = stm32f4::stm32f401, dispatchers = [DMA1_STREAM0,DMA1_STREAM1])]
mod app {
    // Relative app imports
    use app::pmw3389::Pmw3389;
    use app::mouseReport::MouseState;
    use app::pca9624_pw::*;
    use app::hidDescriptors::MouseKeyboard;
    use app::mouseKeyboardReport::MouseKeyboardState;
    use app::macroSystem::*;
    use app::rgb_pattern_things::*;
    // Absolute imports
    use eeprom::EEPROM;
    use stm32f4::stm32f401::*;
    
    use rtt_target::{
        rprintln,
        rtt_init_print
    };
    use dwt_systick_monotonic::*;
    use usb_device::{
        bus::UsbBusAllocator,
        prelude::*,
        endpoint::*
    };
    use usbd_hid::{
        descriptor::{generator_prelude::*},
        hid_class::HIDClass,
    };
    use stm32f4xx_hal::{
        gpio::{Alternate, Output, Pin, PushPull, Speed},
        prelude::*,
        spi::{Spi, TransferModeNormal},
        timer::Delay,
        gpio::*,
        otg_fs::{UsbBus, USB},
        i2c::*,
    };
    
    // Includes for the spi interface
    use embedded_hal::spi::MODE_3;
    use stm32f4::stm32f401::{SPI1, TIM5};


    // Default core clock at 16MHz
    const FREQ_CORE: u32 = 16_000_000;

    #[monotonic(binds = SysTick, default = true)]
    type MyMono = DwtSystick<FREQ_CORE>; // 16MHz cycle accurate accuracy

    
    type Button = ErasedPin<Input<PullDown>>;
    type Scroll = ErasedPin<Input<PullUp>>;
    // Types for spi interface
    type SCK = Pin<Alternate<PushPull, 5_u8>, 'A', 5_u8>;
    type MOSI = Pin<Alternate<PushPull, 5_u8>, 'A', 7_u8>;
    type MISO = Pin<Alternate<PushPull, 5_u8>, 'A', 6_u8>;
    type CS = Pin<Output<PushPull>, 'A', 4_u8>;
    type SPI = Spi<SPI1, (SCK, MISO, MOSI), TransferModeNormal>;
    // Types for pmw3389 device driver
    type DELAY = Delay<TIM5, 1000000_u32>;
    type PMW3389 = Pmw3389<SPI, CS, DELAY>;
    // Types for the i2c interface
    type SCL = Pin<Alternate<OpenDrain, 4_u8>, 'A', 8_u8>;
    type SDA = Pin<Alternate<OpenDrain, 4_u8>, 'C', 9_u8>;
    type I2C = I2c<I2C3, (SCL, SDA)>;
    
    type SERIAL_BUSS<'a> = usbd_serial::SerialPort<'a,UsbBus<USB> > ;
    
    #[shared]
    struct Shared {
        mouse: MouseKeyboardState,
        macro_conf: MacroConfig,
        EXTI : EXTI,
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
        phase_a : Scroll,
        phase_b : Scroll,
        motion : Button,
        ts : u32,
        rgb_pattern_driver : RgbController
    }

    const POLL_INTERVAL_MS: u8 = 1;

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

        let rcc = dp.RCC.constrain();
        let clocks = rcc.cfgr.sysclk(48.MHz()).require_pll48clk().freeze();
        // Grab gpio pins
        let gpioa = dp.GPIOA.split();
        let gpiob = dp.GPIOB.split();
        let gpioc = dp.GPIOC.split();
        // define usb config
        let usb = USB {
            usb_global: dp.OTG_FS_GLOBAL,
            usb_device: dp.OTG_FS_DEVICE,
            usb_pwrclk: dp.OTG_FS_PWRCLK,
            pin_dm: gpioa.pa11.into_alternate(),
            pin_dp: gpioa.pa12.into_alternate(),
            hclk: clocks.hclk(),
        };

        // Configure pmw3389 sensor
        let sck         : SCK       = gpioa.pa5.into_alternate().set_speed(Speed::VeryHigh);
        let miso        : MISO      = gpioa.pa6.into_alternate().set_speed(Speed::High);
        let mosi        : MOSI      = gpioa.pa7.into_alternate().set_speed(Speed::High);
        let cs          : CS        = gpioa.pa4.into_push_pull_output().set_speed(Speed::High);
        let spi         : SPI       = Spi::new(dp.SPI1, (sck, miso, mosi), MODE_3, 1.MHz(), &clocks);
        let delay       : DELAY     = dp.TIM5.delay_us(&clocks);
        let mut pmw3389 : PMW3389   = Pmw3389::new(spi, cs, delay).unwrap();
        // Write the cpi regs on startup
        pmw3389.store_cpi().ok();
        // Defines a i2c interface
        let scl         : SCL       = gpioa.pa8.into_alternate_open_drain();
        let sda         : SDA       = gpioc.pc9.into_alternate_open_drain();
        let mut i2c     : I2C       = I2c::new(dp.I2C3, (scl,sda), Mode::from(1.MHz()), &clocks); 
        
        
        let mut interfaces = standard_interfaces();
        let mut rgb_controller:PCA9624PW = PCA9624PW::new(i2c,interfaces,0x15); // Might be 0xC2
        //rprintln!("rgb controller id {:?}",rgb_controller.whoami());
        // Initiates the pattern controller
        let mut rgb_pattern_driver = RgbController::new(rgb_controller);



        // Configure IO pins
        let mut motion:Button = gpiob.pb13.into_pull_down_input().erase();
        let mut phase_a:Scroll = gpioc.pa12.into_pull_up_input().erase();
        let mut phase_b:Scroll = gpioc.pc11.into_pull_up_input().erase();
        let mut left:Button = gpiob.pb9.into_pull_down_input().erase();
        let mut right:Button = gpiob.pb4.into_pull_down_input().erase();
        let mut middle:Button = gpiob.pb6.into_pull_down_input().erase();
        let mut front:Button = gpiod.pd2.into_pull_down_input().erase();
        let mut back:Button = gpiob.pb7.into_pull_down_input().erase();
        

        phase_a.make_interrupt_source(&mut sys_cfg);
        phase_a.enable_interrupt(&mut dp.EXTI);
        phase_a.trigger_on_edge(&mut dp.EXTI, Edge::RisingFalling);
        phase_b.make_interrupt_source(&mut sys_cfg);
        phase_b.enable_interrupt(&mut dp.EXTI);
        phase_b.trigger_on_edge(&mut dp.EXTI, Edge::RisingFalling);

        //enable_interrupt(&mut phase_a,&mut dp.EXTI,Edge::Rising,&mut sys_cfg);
        //enable_interrupt(&mut phase_b,&mut dp.EXTI,Edge::Rising,&mut sys_cfg);
        enable_interrupt(&mut left,&mut dp.EXTI,Edge::RisingFalling,&mut sys_cfg);
        enable_interrupt(&mut right,&mut dp.EXTI,Edge::RisingFalling,&mut sys_cfg);
        enable_interrupt(&mut middle,&mut dp.EXTI,Edge::RisingFalling,&mut sys_cfg);
        enable_interrupt(&mut front,&mut dp.EXTI,Edge::RisingFalling,&mut sys_cfg);
        enable_interrupt(&mut back,&mut dp.EXTI,Edge::RisingFalling,&mut sys_cfg);
        
        cx.local.bus.replace(UsbBus::new(usb, cx.local.EP_MEMORY));
        // Configure usb class
        let hid = HIDClass::new(
            cx.local.bus.as_ref().unwrap(),
            MouseKeyboard::desc(),
            POLL_INTERVAL_MS,
        );

        // setup macro system
        let mut macro_conf = MacroConfig::new();
        macro_conf.update_config(
            MacroType::MacroSingle(Function::LeftClick),
            MacroType::MacroSingle(Function::RightClick),
            MacroType::MacroSingle(Function::PressKeyboard(4)),
            MacroType::MacroSingle(Function::ScrollUp),
            MacroType::MacroSingle(Function::ScrollDown),
            MacroType::MacroSingle(Function::PressKeyboard(5)),
            MacroType::MacroSingle(Function::PressKeyboard(6)),
        );

        let mouse = MouseKeyboardState::new(pmw3389);
        let usb_dev =
            UsbDeviceBuilder::new(cx.local.bus.as_ref().unwrap(), UsbVidPid(0xc410, 0x000))
                .manufacturer("Ivar och Erik")
                .product("Banger gaming mus")
                .serial_number("1234")
                .max_power(500) // Just take all the power
                .composite_with_iads()      // Define as a composite device
                .build();
        // Enable host wakeup
        usb_dev.remote_wakeup_enabled();
        // It seems that we can install multiple endpoints
        // We would need to denfine 
        let mut EXTI = dp.EXTI;
        let ts = 0;
        (
            Shared {
                mouse,
                macro_conf,
                EXTI
            },
            Local { 
                usb_dev,
                hid,
                left,
                right, 
                middle, 
                front, 
                back, 
                phase_a,
                phase_b,
                motion,
                ts,
                rgb_pattern_driver
            },
            init::Monotonics(mono)
        )
    }
    fn enable_interrupt(btn : &mut Button,  exti : &mut EXTI,edge : stm32f4xx_hal::gpio::Edge,sys_cfg : &mut stm32f4xx_hal::syscfg::SysCfg){
        btn.make_interrupt_source(sys_cfg);
        btn.enable_interrupt(exti);
        btn.trigger_on_edge(exti, edge);

    }

    /// defines a simple pattern loop this should be started at startup when there is some form of pre saved pattern
    #[task(local = [rgb_pattern_driver])]
    fn pattern_itterator(cx : pattern_itterator::Context){
        let mut step = cx.local.rgb_pattern_driver.next_color();
        //pattern_itterator::spawn_after(dwt_systick_monotonic::fugit::Duration(step as u32));
    }


    /**************************
     * Macro system functions *
     **************************/

    #[task(shared = [mouse], priority = 1, capacity = 100)]
    fn end_macro(mut cx: end_macro::Context, f: Function) {
        cx.shared.mouse.lock(|mouse| {
            end_function(f, mouse);
        });
    }

    #[task(shared = [mouse, macro_conf], priority = 1, capacity = 100)]
    fn do_macro(mut cx: do_macro::Context, m: usize, i: usize) {
        cx.shared.macro_conf.lock(|conf| {
            if i == MACRO_SIZE {
                return;
            }
            let (function, delay, time) = conf.get_macro_params(m, i);
            if matches!(function, Function::End) {
                return;
            }
            cx.shared.mouse.lock(|mouse| {
                do_function(function, mouse);
            });
            do_macro::spawn_after(delay.millis(), m, i + 1).unwrap();
            end_macro::spawn_after(time.millis(), function).unwrap();
        });
    }

    #[inline(always)]
    fn handle_macro(conf: &MacroConfig, m: MacroType, mouse: &mut MouseKeyboardState, is_push: bool) {
        rprintln!("handle macro");
        match m {
            MacroType::MacroSingle(f) => {
                match is_push {
                    true => do_function(f, mouse),
                    false => end_function(f, mouse),
                }
            },
            MacroType::MacroMultiple(s) => {
                match is_push {
                    true => {
                        let ms = conf.get_macro_first_delay(s);
                        do_macro::spawn_after(ms.millis(), s, 0).unwrap();
                    }
                    _ => (),
                }
            }
        }
    }
    /******************************
     * End macro system functions *
     ******************************/


    #[task(binds=EXTI9_5,priority = 2, local = [middle, left, back, lf: bool = false, mf: bool = false, bf: bool = false], shared = [mouse, macro_conf])]
    fn middle_left_hand(mut cx: middle_hand::Context) {
        // this should be automatic
        cx.local.middle.clear_interrupt_pending_bit();
        cx.local.left.clear_interrupt_pending_bit();
        if cx.local.middle.is_low() && *cx.local.mf{
            *cx.local.mf = false;
            rprintln!("middle low");
            cx.shared.macro_conf.lock(|conf| {
                cx.shared.mouse.lock(|mouse| {
                    handle_macro(conf, conf.middle_button, mouse, false);
                });
            });
        } else if cx.local.middle.is_high() && !*cx.local.mf {
            rprintln!("middle high");
            *cx.local.mf = true;
            cx.shared.macro_conf.lock(|conf| {
                cx.shared.mouse.lock(|mouse| {
                    handle_macro(conf, conf.middle_button, mouse, true);
                });
            });
        } else if cx.local.left.is_low() && *cx.local.lf {
            rprintln!("left low");
            cx.local.lf = false;
            cx.shared.macro_conf.lock(|conf| {
                cx.shared.mouse.lock(|mouse| {
                    handle_macro(conf, conf.left_button, mouse, false);
                });
            });
        } else if cx.local.left.is_high() && !*cx.local.lf {
            rprintln!("left high");
            cx.local.lf = true;
            cx.shared.macro_conf.lock(|conf| {
                cx.shared.mouse.lock(|mouse| {
                    handle_macro(conf, conf.left_button, mouse, true);
                });
            });
        } else if cx.local.back.is_low() && *cx.local.bf {
            rprintln!("left low");
            cx.local.lf = false;
            cx.shared.macro_conf.lock(|conf| {
                cx.shared.mouse.lock(|mouse| {
                    handle_macro(conf, conf.back_button, mouse, false);
                });
            });
        } else if cx.local.back.is_high() && !*cx.local.bf {
            rprintln!("left high");
            cx.local.lf = true;
            cx.shared.macro_conf.lock(|conf| {
                cx.shared.mouse.lock(|mouse| {
                    handle_macro(conf, conf.back_button, mouse, true);
                });
            });
        }
    }
    
    #[task(binds=EXTI4, local = [right], shared = [mouse, macro_conf])]
    fn right_hand(mut cx: right_hand::Context) {
        // this should be automatic
        cx.local.right.clear_interrupt_pending_bit();

        if cx.local.right.is_low() {
            rprintln!("right low");
            cx.shared.macro_conf.lock(|conf| {
                cx.shared.mouse.lock(|mouse| {
                    handle_macro(conf, conf.right_button, mouse, false);
                });
            });
        } else {
            rprintln!("right high");
            cx.shared.macro_conf.lock(|conf| {
                cx.shared.mouse.lock(|mouse| {
                    handle_macro(conf, conf.right_button, mouse, true);
                });
            });
        }
    }
        
    #[task(binds=EXTI2, local = [front], shared = [mouse, macro_conf, EXTI])]
    fn front_hand(mut cx: front_hand::Context) {
        cx.local.front.clear_interrupt_pending_bit();
        if cx.local.front.is_low() {
            rprintln!("front low");
            cx.shared.macro_conf.lock(|conf| {
                cx.shared.mouse.lock(|mouse| {
                    handle_macro(conf, conf.side_button_front, mouse, false);
                });
            });
        } else {
            rprintln!("front high");
            cx.shared.macro_conf.lock(|conf| {
                cx.shared.mouse.lock(|mouse| {
                    handle_macro(conf, conf.side_button_front, mouse, true);
                });
            });
        }
    }

    #[no_mangle]
    fn delay(td:u32){
        let time = monotonics::now().ticks() as u32;
        while(monotonics::now().ticks() as u32 - time) <  td{}
    }

    #[task(binds=EXTI4, local = [back], shared = [mouse, macro_conf, EXTI])]
    fn back_hand(mut cx: back_hand::Context) {
        // this should be automatic
        cx.local.back.clear_interrupt_pending_bit();
        // Temporarelly disable interrupts
        //cx.shared.EXTI.lock(|EXTI|{
        //    cx.local.back.disable_interrupt(EXTI);
        //});
        //delay(160000);
        //cx.shared.EXTI.lock(|EXTI|{
        //    cx.local.back.enable_interrupt(EXTI);
        //});
        if cx.local.back.is_low() {
            rprintln!("back low");
            cx.shared.macro_conf.lock(|conf| {
                cx.shared.mouse.lock(|mouse| {
                    handle_macro(conf, conf.side_button_back, mouse, false);
                });
            });
        } else {
            rprintln!("back high");
            cx.shared.macro_conf.lock(|conf| {
                cx.shared.mouse.lock(|mouse| {
                    handle_macro(conf, conf.side_button_back, mouse, true);
                });
            });
        }
    }

    // interrupt generated each time the hid device is polled
    // in this example each 1ms (POLL_INTERVAL_MS = 1)
    #[task(
        binds=OTG_FS,
        priority = 1,
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
        let mut buf = [0u8; 8];
        match hid.pull_raw_output(&mut buf).ok(){
            // Should return almost istantaneously if there is no data
            Some(len) => {
                // The mouse has been polled for update purposes
                rprintln!("{:?}",buf);
                handle_host_call::spawn(buf);
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
    #[task(shared = [macro_conf])]
    fn handle_host_call(mut cx :handle_host_call::Context,buffer : [u8; 8]) {
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
                cx.shared.macro_conf.lock(|conf| {
                    conf.handle_binary_config(&buffer);
                });
            },
            _ => {
                rprintln!("unknown");
            }
        }

    }
    #[task(shared = [mouse])]
    fn handle_dpi(mut cx : handle_dpi::Context,dpi : u16){
        rprintln!("{:}",dpi);
        cx.shared.mouse.lock(|mouse| {
            mouse.write_dpi(dpi);
        });
    }
    #[idle(shared = [mouse])]
    fn idle(mut cx: idle::Context) -> ! {
        loop {
            cx.shared.mouse.lock(|mouse|{
                mouse.read_sensor();
            });   
        }
    }
}
