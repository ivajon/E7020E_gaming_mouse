// cargo run --example rtt_rtic_hello
// Assumes cargo/config.toml
// runner = "probe-run --chip STM32F411RETx"

#![no_main]
#![no_std]

use panic_halt as _;

#[rtic::app(device = stm32f4::stm32f401)]
mod app {
    use rtt_target::{rprintln, rtt_init_print};

    #[shared]
    struct Shared {}

    #[local]
    struct Local {}

    #[init]
    fn init(_: init::Context) -> (Shared, Local, init::Monotonics) {
        rtt_init_print!();
        for i in 0..=12 {
            rprintln!("RTIC Says Hello, world {}!!", i);
        }

        (Shared {}, Local {}, init::Monotonics())
    }

    #[idle]
    fn idle(_: idle::Context) -> ! {
        rprintln!("lets get lazy");
        loop {}
    }
}
