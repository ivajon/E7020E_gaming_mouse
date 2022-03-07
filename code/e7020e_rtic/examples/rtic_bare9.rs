//! bare9.rs
//!
//! Serial
//!
//! What it covers:
//! - serial communication
//! - good design
//!
//! In this case we use RTT, so change your runner accordingly in `.cargo/config`.
#![no_main]
#![no_std]

use panic_rtt_target as _;

#[rtic::app(device = stm32f4::stm32f411, dispatchers = [EXTI0])]
mod app {

    use rtt_target::{rprintln, rtt_init_print};
    use stm32f4xx_hal::gpio::*;

    use stm32f4xx_hal::{
        prelude::*,
        serial::{config::Config, Event, Rx, Serial, Tx},
    };

    use nb::block;

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

        let mut serial: Serial<USART2, _> = Serial::new(
            device.USART2,
            (tx_pin, rx_pin),
            Config::default().baudrate(115_200.bps()),
            &clocks,
        )
        .unwrap();

        // generate interrupt on Rxne
        serial.listen(Event::Rxne);

        let (tx, rx) = serial.split();

        (Shared { tx, rx }, Local {}, init::Monotonics())
    }

    #[idle]
    fn idle(_cx: idle::Context) -> ! {
        rprintln!("idle");

        loop {
            // here your application may run some background task
        }
    }

    // capacity sets the size of the input buffer (# outstanding messages)
    #[task(priority = 1, shared = [tx], capacity = 128)]
    fn worker(cx: worker::Context, data: u8) {
        let tx = cx.shared.tx;
        tx.write(data).unwrap();
        rprintln!("data {}", data);
    }

    // Task bound to the USART2 interrupt.
    #[task(binds = USART2, priority = 2, shared = [rx])]
    fn usart2(cx: usart2::Context) {
        let rx = cx.shared.rx;
        let data = rx.read().unwrap();
        worker::spawn(data).unwrap();
    }
}

// 0. Background
//
//    As seen in the prior example, you may loose data unless polling frequently enough.
//    Let's try an interrupt driven approach instead.
//
//    In init we just add:
//
//    // generate interrupt on Rxne
//    serial.listen(Event::Rxne);
//
//    This causes the USART hardware to generate an interrupt when data is available.
//
//    // Task bound to the USART2 interrupt.
//    #[task(binds = USART2,  priority = 2, shared = [rx])]
//    fn usart2(cx: usart2::Context) {
//        let rx = cx.shared.rx;
//        let data = rx.read().unwrap();
//        worker::spawn(data).unwrap();
//    }
//
//    The `usart2` task will be triggered, and we read one byte from the internal
//    buffer in the USART2 hardware. (panic if something goes bad)
//
//    We send the read byte to the `worker` task (by `worker::spawn(data).unwrap();`)
//    (We panic if the capacity of the message queue is reached)
//
//    // capacity sets the size of the input buffer (# outstanding messages)
//    #[task(priority = 1, shared = [tx], capacity = 128)]
//    fn worker(cx: worker::Context, data: u8) {
//        let tx = cx.shared.tx;
//        tx.write(data).unwrap();
//        rprintln!("data {}", data);
//    }
//
//    Here we echo the data back, `tx.write(data).unwrap();` (panic if usart is busy)
//    We then trace the received data `rprintln!("data {}", data);`
//
//    The `priority = 2` gives the `usart2` task the highest priority
//    (to ensure that we don't miss data).
//
//    The `priority = 1` gives the `worker` task a lower priority.
//    Here we can take our time and process the data.
//
//    `idle` runs at priority 0, lowest priority in the system.
//    Here we can do some background job, when nothing urgent is happening.
//
//    This is an example of a good design!
//    (However, there is still room for improvement, e.g. regarding error handling.)
//
// 1. In this example we use RTT.
//
//    > cargo run --example rtic_bare9 --release
//
//    Try breaking it!!!!
//    Throw any data at it, and see if you could make it panic!
//
//    Were you able to crash it?
//
//    ** your answer here **
//
//    Commit your answer (bare9_1)
//
// 2. Now, re-implement the received and error counters from previous exercise.
//
//    Good design:
//    - Defer any tracing to lower priority task/tasks
//
//    - State variables can be introduced as task local resources.
//
//    For details see the RTIC book.
//    https://rtic.rs/1/book/en/by-example/resources.html
//
//    You can send the content of a local resource to another task.
//    Useful e.g., to track number of received bytes.
//
//    Message passing never introduces any locks (good for real-time systems).
//    Useful e.g., to pass the data and number of bytes received to the `worker`.
//
//    Test that your implementation works and traces number of
//    bytes received and errors encountered.
//
//    If implemented correctly, it should be very hard (or even impossible)
//    to get an error.
//
//    Hint: You may use the `block!(tx.write...)` to ensure that you don't saturate
//    the single byte buffer on the tx usart. The `block` will "busy wait" until
//    the byte has been written or some other error occurs.
//
//    You can force errors by faking workload (e.g., burning clock cycles like
//    we did in previous labs).
//
//    Try to see how much "slack" you have in the two tasks (`worker` and `usart2`).
//    (I.e., how much extra workload you can throw in until you start seeing problems.)
//
//    Describe in your own words the relation between "slack"
//    and "capacity" for the `worker` task. (So play around tuning these parameters.)
//
//    ** your answer here **
//
//    Once finished, comment your code.
//
//    Commit your code (bare9_2)
//
// 3. Discussion
//
//    Here you have used RTIC to implement a highly efficient and good design.
//
//    Tasks in RTIC are run-to-end, with non-blocking access to resources.
//
//    Tasks in RTIC are scheduled according to priorities.
//    (A higher priority task `H` always preempts lower priority task `L` running,
//    unless `L` holds a resource with higher or equal ceiling as `H`.)
//
//    Tasks in RTIC can spawn other tasks.
//    (`capacity` sets the message queue size.)
//
//    By design RTIC guarantees race- and deadlock-free execution.
//
//    It also comes with theoretical underpinning for static analysis.
//    - task response time
//    - overall schedulability
//    - stack memory analysis
//    - etc.
//
//    RTIC leverages on the zero-cost abstractions in Rust,
//    and the implementation offers best in class performance.
