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
    use app::pca9624_pw::*;
    use app::rgb_pattern_things::*;
    // Absolute imports
    use eeprom::EEPROM;
    use stm32f4::stm32f401::*;
    
    use rtt_target::{
        rprintln,
        rtt_init_print
    };
    use dwt_systick_monotonic::*;
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
    use stm32f4::stm32f401::{SPI2, TIM5};


    // Default core clock at 16MHz
    const FREQ_CORE: u32 = 16_000_000;

    #[monotonic(binds = SysTick, default = true)]
    type MyMono = DwtSystick<FREQ_CORE>; // 16MHz cycle accurate accuracy

    
    // Types for pmw3389 device driver
    type DELAY = Delay<TIM5, 1000000_u32>;
    // Types for the i2c interface
    type SCL = Pin<Alternate<OpenDrain, 4_u8>, 'A', 8_u8>;
    type SDA = Pin<Alternate<OpenDrain, 4_u8>, 'C', 9_u8>;
    type I2C = I2c<I2C3, (SCL, SDA)>;
    #[shared]
    struct Shared {
    }
    
    #[local]
    struct Local {
    }

    const POLL_INTERVAL_MS: u8 = 1;

    #[init()]
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

        // Defines a i2c interface
        let mut scl         : SCL       = gpioa.pa8.into_alternate_open_drain();
        let mut sda         : SDA       = gpioc.pc9.into_alternate_open_drain();
        let mut i2c     : I2C       = I2c::new(dp.I2C3, (scl,sda), Mode::from(100.kHz()), &clocks); 
        
        // testing stuff
        let b = i2c.read(0x01)?;
        i2c.write(0x01, &[(1 << 3) | b as u8]);

        i2c.write(0x02, &[0xff]);
        
        let mut interfaces = standard_interfaces();
        //let mut rgb_controller:PCA9624PW = PCA9624PW::new(i2c,interfaces,0x00); // Might be 0xC2
        //rprintln!("rgb controller id {:?}",rgb_controller.whoami());
        // Initiates the pattern controller
        rprintln!("before who am i");
        //rprintln!("rgb controller id {:?}",rgb_controller.whoami());
        // Initiates the pattern controller

        //rgb_controller.set_colour_from_values(0, 0xff, 0xff, 0xff);
        //rgb_controller.write_colours(0);
        //rgb_controller.set_colour_from_values(1, 0xff, 0xff, 0xff);
        //rgb_controller.write_colours(1);

        //let mut rgb_pattern_driver = RgbController::new(rgb_controller);

        (
            Shared {
            },
            Local { 
            },
            init::Monotonics(mono)
        )
    }

    /// defines a simple pattern loop this should be started at startup when there is some form of pre saved pattern
    //#[task(local = [])]
    //fn pattern_itterator(cx : pattern_itterator::Context){
    //    let mut step = cx.local.rgb_pattern_driver.next_color();
    //    //pattern_itterator::spawn_after(dwt_systick_monotonic::fugit::Duration(step as u32));
    //}


    #[idle(shared = [])]
    fn idle(mut cx: idle::Context) -> ! {
        loop {
        }
    }
}
