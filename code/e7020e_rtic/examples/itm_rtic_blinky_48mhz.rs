// From terminal:
// cargo run --example itm_rtic_blinky_48mhz
//
// Assumes cargo/config.toml
// runner = "arm-none-eabi-gdb -q -x openocd.gdb"
//
// Assumes `openocd` running in separate terminal
// > openocd -f openocd.cfg
//
// Assumes `itmdump` running in separate terminal
// >mkfifo /tmp/itm.fifo
// >itmdump -F -f /tmp/itm.fifo
//
// From vscode:
// Assumes (Cortex Release 48MHz)
// Press F5             (to compile/flash/debug the application)
// TERMINAL->gdb-server (to view semihosting trace in the openocd session)
// TERMINAL->SWO:ITM... (to view ITM trace)
// DEBUG CONSOLE        (to view/control the gdb session)
#![no_main]
#![no_std]

use panic_semihosting as _;

#[rtic::app(device = stm32f4::stm32f411, dispatchers = [EXTI0, EXTI1])]
mod app {
    use cortex_m::{iprint, iprintln};
    use dwt_systick_monotonic::*;
    use stm32f4xx_hal::prelude::*;

    // Core clock at 48MHz
    const FREQ_CORE: u32 = 48_000_000;

    #[monotonic(binds = SysTick, default = true)]
    type MyMono = DwtSystick<FREQ_CORE>; // 48MHz cycle accurate accuracy

    #[shared]
    struct Shared {
        #[lock_free] // shared between tasks at the same priority
        gpioa: stm32f4::stm32f411::GPIOA,

        // shared between tasks at different priorities (idle 0, log 1)
        itm: cortex_m::peripheral::ITM,
    }

    #[local]
    struct Local {}

    #[init]
    fn init(mut cx: init::Context) -> (Shared, Local, init::Monotonics) {
        let systick = cx.core.SYST;
        let mut dcb = cx.core.DCB;
        let dwt = cx.core.DWT;
        let rcc = cx.device.RCC;

        // enable gpioa
        rcc.ahb1enr
            .write(|w| w.gpioaen().enabled().gpioben().enabled());

        // set core clock to 48MHz and freeze the settings
        let clk = rcc.constrain().cfgr.sysclk(48.Mhz()).freeze();

        // Initialize the monotonic (SysTick driven by core clock)
        let mono = DwtSystick::new(&mut dcb, dwt, systick, clk.sysclk().raw());

        let stim = &mut cx.core.ITM.stim;
        iprintln!(&mut stim[0], "init");

        let gpioa = cx.device.GPIOA;

        // set mode
        gpioa.moder.write(|w| w.moder5().output());

        led_on::spawn().ok();
        log::spawn().ok();

        (
            Shared {
                gpioa,
                itm: cx.core.ITM,
            },
            Local {},
            init::Monotonics(mono),
        )
    }

    #[idle(shared = [itm])]
    fn idle(mut cx: idle::Context) -> ! {
        cx.shared.itm.lock(|itm| {
            let stim = &mut itm.stim;
            iprintln!(&mut stim[0], "idle");
        });

        loop {}
    }

    #[task(shared = [itm])]
    fn log(mut cx: log::Context) {
        cx.shared.itm.lock(|itm| {
            let stim = &mut itm.stim;
            iprint!(&mut stim[0], ".");
        });
        log::spawn_after(1.secs()).ok();
    }

    #[task(priority = 2, shared = [gpioa])]
    fn led_on(cx: led_on::Context) {
        cx.shared.gpioa.bsrr.write(|w| w.bs5().set_bit());
        led_off::spawn_after(1.millis()).ok();
    }

    #[task(priority = 2, shared = [gpioa])]
    fn led_off(cx: led_off::Context) {
        cx.shared.gpioa.bsrr.write(|w| w.br5().set_bit());
        led_on::spawn_after(9.millis()).ok();
    }
}

// In this example the LED on time is 1ms and the off time 9ms
// This amounts to a PWM with a period of 10ms with a duty cycle 1/10.
//
// Try different duty cycles, e.g., 5/10, 9/10 and see
// how the LED intensity changes.
//
// Try different PWM periods, e.g. 1000ms.
// Then you will see the LED blinking.
