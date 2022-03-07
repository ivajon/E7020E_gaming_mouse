// From terminal:
// cargo run --example rtic_hello
// Assumes cargo/config.toml
// runner = "arm-none-eabi-gdb -q -x openocd.gdb"
// Assumes `openocd` running in separate terminal
// > openocd -f openocd.cfg
//
// From vscode:
// Press F5 (to compile/flash/debug the application)
// TERMINAL->gdb-server (to view semihosting trace in the openocd session)
// DEBUG CONSOLE (to view/control the gdb session)

#![no_main]
#![no_std]

use panic_halt as _;

#[rtic::app(device = stm32f4::stm32f411)]
mod app {
    use cortex_m_semihosting::hprintln;

    #[shared]
    struct Shared {}

    #[local]
    struct Local {}

    #[init]
    fn init(_: init::Context) -> (Shared, Local, init::Monotonics) {
        for a in 1..=12 {
            hprintln!("RTIC Says Hello, to all students!! {}", a).unwrap();
        }

        (Shared {}, Local {}, init::Monotonics())
    }
}
