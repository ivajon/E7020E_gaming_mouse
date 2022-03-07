// From terminal:
// cargo run --example itm_rtic_hello
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
// -s, --stimulus <port>    Stimulus port to extract ITM data for. [default: 0]
//
// From vscode:
// Assumes (Cortex Debug)
// Press F5             (to compile/flash/debug the application)
// TERMINAL->gdb-server (to view semihosting trace in the openocd session)
// TERMINAL->SWO:ITM... (to view ITM trace)
// DEBUG CONSOLE        (to view/control the gdb session)

#![no_main]
#![no_std]

use panic_halt as _;

#[rtic::app(device = stm32f4::stm32f411)]
mod app {
    use cortex_m::iprintln;

    #[shared]
    struct Shared {}

    #[local]
    struct Local {}

    #[init]
    fn init(cx: init::Context) -> (Shared, Local, init::Monotonics) {
        let mut core = cx.core;
        let stim = &mut core.ITM.stim;
        for a in 1..=12 {
            iprintln!(&mut stim[0], "RTIC Says Hello, to all students!! {}", a);
            if a % 3 == 0 {
                iprintln!(&mut stim[1], "Hello to each 3rd {}", a);
            }
        }

        (Shared {}, Local {}, init::Monotonics())
    }
}
