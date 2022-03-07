//! bare8.rs
//!
//! Serial
//!
//! What it covers:
//! - serial communication
//! - bad design
//!
//! In this case we use RTT, so change your runner accordingly in `.cargo/config`.
#![no_main]
#![no_std]

use panic_rtt_target as _;

#[rtic::app(device = stm32f4::stm32f411, dispatchers = [EXTI0])]
mod app {

    use nb::block;
    use rtt_target::{rprintln, rtt_init_print};
    use stm32f4xx_hal::gpio::*;

    use stm32f4xx_hal::{
        prelude::*,
        serial::{config::Config, Rx, Serial, Tx},
    };

    use stm32f4::stm32f411::USART2;

    #[shared]
    struct Shared {
        #[lock_free]
        tx: Tx<USART2>,

        #[lock_free]
        rx: Rx<USART2>,
    }

    #[local]
    struct Local {}

    #[init]
    fn init(cx: init::Context) -> (Shared, Local, init::Monotonics) {
        rtt_init_print!();
        rprintln!("init");

        let device = cx.device;

        let pa = device.GPIOA.split();

        // set to alternate mode af7 for serial port configuration
        let tx_pin = pa.pa2.into_alternate::<7>();
        let rx_pin = pa.pa3.into_alternate::<7>();

        // 16 MHz (default, all clocks)
        let rcc = device.RCC.constrain();
        let clocks = rcc.cfgr.freeze();

        let serial: Serial<USART2, _> = Serial::new(
            device.USART2,
            (tx_pin, rx_pin),
            Config::default().baudrate(115_200.bps()),
            &clocks,
        )
        .unwrap();

        let (tx, rx) = serial.split();

        (Shared { tx, rx }, Local {}, init::Monotonics())
    }

    #[idle(shared = [tx, rx])]
    fn idle(cx: idle::Context) -> ! {
        rprintln!("idle");
        let rx = cx.shared.rx;
        let tx = cx.shared.tx;

        loop {
            match block!(rx.read()) {
                Ok(byte) => {
                    rprintln!("Ok {:?}", byte);
                    tx.write(byte).unwrap();
                }
                Err(err) => {
                    rprintln!("Error {:?}", err);
                }
            }
        }
    }
}

// 0. Background
//
//    The Nucleo st-link programmer provides a Virtual Com Port (VCP).
//    It is connected to the PA2(TX)/PA3(RX) pins of the stm32f401/411.
//    On the host, the VCP is presented under `/dev/ttyACMx`, where
//    `x` is an enumerated number (ff 0 is busy it will pick 1, etc.)
//
// 1. In this example we use RTT.
//
//    > cargo run --example rtic_bare8
//
//    Start a terminal program, e.g., `cutecom`.
//    Connect to the port
//
//    Device       /dev/ttyACM0
//    Baude Rate   115200
//    Data Bits    8
//    Stop Bits    1
//    Parity       None
//
//    This setting is typically abbreviated as 115200 8N1.
//
//    Send a single character (byte),
//    Use the "Input" field to write data to send.
//    You can choose how [return] should be sent in the dropdown right of the "Input" field.
//
//    Verify that sent bytes are echoed back, and that RTT tracing is working.
//
//    Try sending "a", don't send the quotation marks, just a.
//
//    What do you receive in `cutecom`?
//
//    ** your answer here **
//
//    What do you receive in the RTT terminal?
//
//    ** your answer here **
//
//    Try sending: "abcd" as a single sequence, don't send the quotation marks, just abcd.
//
//    What did you receive in `cutecom`?
//
//    ** your answer here **
//
//    What do you receive in the RTT terminal?
//
//    ** your answer here **
//
//    What do you believe to be the problem?
//
//    Hint: Look at the code in `idle` what does it do?
//
//    ** your answer here **
//
//    Experiment a bit, what is the max length sequence you can receive without errors?
//
//    ** your answer here **
//
//    Commit your answers (bare8_1)
//
// 2. Add a local variable `received` that counts the number of bytes received.
//    Add a local variable `errors` that counts the number of errors.
//
//    Adjust the RTT trace to print the added information inside the loop.
//
//    Compile/run reconnect, and verify that it works as intended.
//
//    Commit your development (bare8_2)
//
// 3. Experiment a bit, what is the max length sequence you can receive without errors?
//
//    ** your answer here **
//
//    How did the added tracing/instrumentation affect the behavior?
//
//    ** your answer here **
//
//    Commit your answer (bare8_3)
//
// 4. Now try compile and run the same experiment 3 but in --release mode.
//
//    > cargo run --example rtic_bare8 --release
//
//    Reconnect your `cutecom` terminal.
//
//    Experiment a bit, what is the max length sequence you can receive without errors?
//
//    ** your answer here **
//
//    Commit your answer (bare8_4)
//
// 5. Discussion
//
//    (If you ever used Arduino, you might feel at home with the `loop` and poll design.)
//
//    Typically, this is what you can expect from a polling approach, if you
//    are not very careful what you are doing performance will be terrible.
//    This exemplifies a bad design.
//
//    Loss of data might be Ok for some applications but this typically NOT what we want.
//
//    (With that said, Arduino gets away with some simple examples as their drivers do
//    internal magic - buffering data etc.)
