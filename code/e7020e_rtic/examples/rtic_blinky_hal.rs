// From terminal:
// cargo run --example rtic_blinky_hal
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

use panic_semihosting as _;

#[rtic::app(device = stm32f4::stm32f411, dispatchers = [EXTI0])]
mod app {
    use cortex_m_semihosting::hprintln;
    use dwt_systick_monotonic::*;
    use stm32f4xx_hal::gpio::*;

    type LED = Pin<Output<PushPull>, 'A', 5>;

    // Default core clock at 16MHz
    const FREQ_CORE: u32 = 16_000_000;

    #[monotonic(binds = SysTick, default = true)]
    type MyMono = DwtSystick<FREQ_CORE>; // 16MHz cycle accurate accuracy

    #[shared]
    struct Shared {
        #[lock_free] // shared between tasks at the same priority
        led: LED,
    }

    #[local]
    struct Local {}

    #[init]
    fn init(cx: init::Context) -> (Shared, Local, init::Monotonics) {
        hprintln!("init").ok();

        let systick = cx.core.SYST;
        let mut dcb = cx.core.DCB;
        let dwt = cx.core.DWT;

        // Initialize the monotonic (SysTick driven by core clock)
        let mono = DwtSystick::new(&mut dcb, dwt, systick, FREQ_CORE);

        let gpioa = cx.device.GPIOA;

        let pa = gpioa.split();
        let led = pa.pa5.into_push_pull_output();

        led_on::spawn().ok();

        (Shared { led }, Local {}, init::Monotonics(mono))
    }

    #[idle]
    fn idle(_cx: idle::Context) -> ! {
        hprintln!("idle").ok();

        loop {}
    }

    #[task(shared = [led])]
    fn led_on(cx: led_on::Context) {
        hprintln!("led_on").ok();
        cx.shared.led.set_high();
        led_off::spawn_after(1.secs()).ok();
    }

    #[task(shared = [led])]
    fn led_off(cx: led_off::Context) {
        hprintln!("led_off").ok();
        cx.shared.led.set_low();
        led_on::spawn_after(1.secs()).ok();
    }
}
