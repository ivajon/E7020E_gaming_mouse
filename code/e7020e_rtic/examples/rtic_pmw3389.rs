//! examples/rtt-pmw3389.rs
//! cargo run --examples rtt-pmw3389 --release
//!
//! In this case we use RTT, so change your runner accordingly in `.cargo/config`.

#![no_main]
#![no_std]

use panic_rtt_target as _;
#[rtic::app(device = stm32f4::stm32f411)]
mod app {
    use app::pmw3389::Pmw3389;
    use dwt_systick_monotonic::*;

    use embedded_hal::spi::MODE_3;

    use rtt_target::{rprintln, rtt_init_print};

    use stm32f4xx_hal::{
        gpio::{Alternate, Output, Pin, PushPull, Speed},
        prelude::*,
        spi::{Spi, TransferModeNormal},
        timer::Delay,
    };

    use stm32f4::stm32f411::{SPI2, TIM5};

    // types need to be concrete for storage in a resource
    type SCK = Pin<Alternate<PushPull, 5_u8>, 'B', 10_u8>;
    type MOSI = Pin<Alternate<PushPull, 5_u8>, 'C', 3_u8>;
    type MISO = Pin<Alternate<PushPull, 5_u8>, 'C', 2_u8>;
    type CS = Pin<Output<PushPull>, 'B', 4_u8>;

    type SPI = Spi<SPI2, (SCK, MISO, MOSI), TransferModeNormal>;
    type DELAY = Delay<TIM5, 1000000_u32>;
    type PMW3389 = Pmw3389<SPI, CS, DELAY>;

    // Core clock at 48MHz
    const FREQ_CORE: u32 = 48_000_000;

    #[monotonic(binds = SysTick, default = true)]
    type MyMono = DwtSystick<FREQ_CORE>; // 48MHz cycle accurate accuracy

    #[shared]
    struct Shared {}

    #[local]
    struct Local {
        pmw3389: PMW3389,
    }

    #[init]
    fn init(cx: init::Context) -> (Shared, Local, init::Monotonics) {
        rtt_init_print!();
        rprintln!("init");
        let systick = cx.core.SYST;
        let mut dcb = cx.core.DCB;
        let dwt = cx.core.DWT;
        let device = cx.device;
        let rcc = device.RCC;

        // set core clock to 48MHz and freeze the settings
        let clocks = rcc.constrain().cfgr.sysclk(48.MHz()).freeze();

        // Initialize the monotonic (SysTick driven by core clock)
        let _mono = DwtSystick::new(&mut dcb, dwt, systick, clocks.sysclk().raw());

        // Create a delay abstraction based on general-purpose 32-bit timer TIM5
        let delay: DELAY = device.TIM5.delay_us(&clocks);

        // Configure SPI, colors just for the demo pcb showed in class
        // spi2
        // sck    - pb10, (yellow)
        // miso   - pc2, (red)
        // mosi   - pc3, (orange)
        // ncs    - pb4, (white)
        // motion - (brown)
        //
        // +5, (white)
        // gnd, (black)

        let gpiob = device.GPIOB.split();
        let gpioc = device.GPIOC.split();

        let sck: SCK = gpiob.pb10.into_alternate().set_speed(Speed::VeryHigh);
        let miso: MISO = gpioc.pc2.into_alternate().set_speed(Speed::High);
        let mosi: MOSI = gpioc.pc3.into_alternate().set_speed(Speed::High);
        let cs: CS = gpiob.pb4.into_push_pull_output().set_speed(Speed::High);
        let spi: SPI = Spi::new(device.SPI2, (sck, miso, mosi), MODE_3, 1.MHz(), &clocks);

        let pmw3389: PMW3389 = Pmw3389::new(spi, cs, delay).unwrap();

        (Shared {}, Local { pmw3389 }, init::Monotonics(_mono))
    }

    #[idle(local = [pmw3389])]
    fn idle(cx: idle::Context) -> ! {
        let pmw3389 = cx.local.pmw3389;
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
        pmw3389.set_cpi(8000).unwrap();

        let mut x_acc: i32 = 0;
        loop {
            let status = pmw3389.read_status().unwrap();
            x_acc += status.dx as i32;
            rprintln!("acc {} dx, dy = {:?}", x_acc, status);

            pmw3389.delay.delay_ms(100_u32);
        }
    }
}
