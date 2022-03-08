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
    use embedded_hal::blocking::i2c;
    use stm32f4xx_hal::i2c::I2c3;

    #[shared]
    struct Shared {}

    #[local]
    struct Local {}

    #[init]
    fn init(cx: init::Context) -> (Shared, Local, init::Monotonics) {
        for a in 1..=12 {
            hprintln!("RTIC Says Hello, to all students!! {}", a).unwrap();
        }

        // running on I2C3 pins PC9 = SDA & SCL = PA8
        /*
        let dp = cx.device;
        let i2c_dp = dp.I2C3;
        let gpioa = dp.GPIOA.split();
        let gpioc = dp.GPIOC.split();
        let sda_pin = gpioc.pc9.into_alternate::I2C3_SDA();
        let scl_pin = gpioa.pa8.into_alternate::I2C3_SCL();
        */

        let i2c = I2C3::new();

        (Shared {}, Local {}, init::Monotonics())
    }
}
