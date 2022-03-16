// From terminal:
// cargo run --example rtic_blinky
// Assumes cargo/config.toml
// runner = "arm-none-eabi-gdb -q -x openocd.gdb"
// Assumes `openocd` running in separate terminal
// > openocd -f openocd.cfg
//
// From vscode:
// Press F5             (to compile/flash/debug the application)
// TERMINAL->gdb-server (to view semihosting trace in the openocd session)
// DEBUG CONSOLE        (to view/control the gdb session)
#![no_main]
#![no_std]

use panic_halt as _;

#[rtic::app(device = stm32f4::stm32f401, dispatchers = [EXTI0])]
mod app {
    use cortex_m_semihosting::hprintln;
    use dwt_systick_monotonic::*;

    use embedded_hal::spi::MODE_3;

    use rtt_target::{rprintln, rtt_init_print};

    use stm32f4xx_hal::{
        gpio::{Alternate, Output, Pin, PushPull, Speed},
        prelude::*,
        i2c::{I2c, DutyCycle, Mode},
        timer::Delay,
    };
    use dwt_systick_monotonic::*;
    use stm32f4xx_hal::gpio::*;
    use stm32f4::stm32f401::*;
    // Default core clock at 16MHz
    const FREQ_CORE: u32 = 16_000_000;

    #[monotonic(binds = SysTick, default = true)]
    type MyMono = DwtSystick<FREQ_CORE>; // 16MHz cycle accurate accuracy

    #[shared]
    struct Shared {
        #[lock_free] // shared between tasks at the same priority
        gpioa: stm32f4::stm32f401::GPIOA,
        
    }

    type SCL = Pin<Alternate<PushPull, 5_u8>, 'A', 8_u8>;
    type SDA = Pin<Alternate<PushPull, 5_u8>, 'C', 9_u8>;
    #[local]
    struct Local {}

    #[init]
    fn init(cx: init::Context) -> (Shared, Local, init::Monotonics) {
        let peripherals = Peripherals::take().unwrap();
        hprintln!("init").ok();
        let systick = cx.core.SYST;
        let mut dcb = cx.core.DCB;
        let dwt = cx.core.DWT;

        // Initialize the monotonic (SysTick driven by core clock)
        let mono = DwtSystick::new(&mut dcb, dwt, systick, FREQ_CORE);

        let rcc = cx.device.RCC;
        // Defines a clock speed
        let clocks = rcc.constrain().cfgr.sysclk(48.MHz()).freeze();

        let gpioa = cx.device.GPIOA;
        let gpioc = cx.device.GPIOC;
        let gpa = gpioa.split();
        let gpc = gpioc.split();
        let mut scl : SCL = gpa.pa8.into_alternate();
        let mut sda : SDA = gpc.pc9.into_alternate();
        // Instantiate a new i2c bus from sda,scl using the standard mode


        
        let i2c = I2c::new(peripherals.I2C3, 
            (sda,scl),
             Mode::Standard{frequency: 400.Hz()}, &clocks);
        use stm32f4xx_hal::gpio::*;
        // enable gpioa
        rcc.ahb1enr.write(|w| w.gpioaen().enabled());

        // set mode
        gpioa.moder.write(|w| w.moder5().output());

        led_on::spawn().ok();

        (Shared { gpioa }, Local {}, init::Monotonics(mono))
    }

    #[idle]
    fn idle(_cx: idle::Context) -> ! {
        hprintln!("idle").ok();

        loop {}
    }

    #[task(shared = [gpioa])]
    fn led_on(cx: led_on::Context) {
        hprintln!("led_on").ok();
        cx.shared.gpioa.bsrr.write(|w| w.br5().set_bit());
        led_off::spawn_after(1.secs()).ok();
    }

    #[task(shared = [gpioa])]
    fn led_off(cx: led_off::Context) {
        hprintln!("led_off").ok();
        cx.shared.gpioa.bsrr.write(|w| w.bs5().set_bit());
        led_on::spawn_after(1.secs()).ok();
    }
}
