//! bare9_no_block.rs
//!
//! Serial
//!
//! What it covers:
//! - better logging
//!
//! - serial communication
//! - good design
//!
//!
//! In this case we use RTT, so change your runner accordingly in `.cargo/config`.
#![no_main]
#![no_std]

use panic_rtt_target as _;

#[rtic::app(device = stm32f4::stm32f411, dispatchers = [EXTI0])]
mod app {
    use core::fmt::Write;

    use rtt_target::{rtt_init, UpChannel};
    use stm32f4xx_hal::gpio::*;

    use stm32f4xx_hal::{
        prelude::*,
        serial::{config::Config, Event, Rx, Serial, Tx},
    };

    use stm32f4::stm32f411::USART2;

    #[shared]
    struct Shared {
        #[lock_free]
        tx: Tx<USART2>,

        #[lock_free]
        rx: Rx<USART2>,

        #[lock_free]
        idle: UpChannel,

        #[lock_free]
        log: UpChannel,
    }

    #[local]
    struct Local {}

    #[init]
    fn init(cx: init::Context) -> (Shared, Local, init::Monotonics) {
        let mut channels = rtt_init!(
            up: {
                0: {
                    size: 128
                    name:"Idle"
                }
                1: {
                    size: 128
                    name:"Log"
                }
            }
        );
        writeln!(channels.up.0, "init");

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

        (
            Shared {
                idle: channels.up.0,
                log: channels.up.1,
                tx,
                rx,
            },
            Local {},
            init::Monotonics(),
        )
    }

    #[idle(shared = [idle])]
    fn idle(mut cx: idle::Context) -> ! {
        writeln!(cx.shared.idle, "idle");

        loop {
            // here your application may run some background task
        }
    }

    // capacity sets the size of the input buffer (# outstanding messages)
    #[task(priority = 1, shared = [tx, log], capacity = 128)]
    fn worker(mut cx: worker::Context, data: u8) {
        let tx = cx.shared.tx;
        tx.write(data).unwrap();
        writeln!(
            cx.shared.log,
            "data {} with {:#08b} lots {:o} of {:#08X} formatting {:#08x}",
            data, data, data, data, data
        )
        .unwrap();
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
//    If you do a lot of formatting you may have noticed that you
//    may start to loose performance.
//
//    The reason is that you are using singe rtt channel. To prevent
//    data race on the channel a global critical section is used.
//
//    In essence the interrupts are disabled and BANG!!!!
//
//    RTIC to our saviour.
//
//    You can split the RTT into different channels and delegate
//    these to out tasks.
//
//    Notice, we only one channel per priority level (tasks at the same
//    priority won't preempt each other right).
//
//    The `probe-run` tool that you have used so far is targeting
//    the beginner, making it easy for you to get started.
//
//    The `cargo-embed` tool is much more versatile, but requires some
//    configuration.
//
//    `rtt_init!` is used to setup the RTT channels on the target side.
//
//    The `Embed.toml` is used to setup the RTT channels on the host side.
//    See the example `Embed.toml`
//
//    We have set the `up_mode = "BlockIfFull"` on the `up = 1` channel,
//    on the host side, which means that if the host and target are
//    connected, the target will block if the buffer is full.
//    If there is no host connected, the target will just ignore the
//    case when the buffer gets full. So this is a good behavior.
//
//    Installing.
//    You can install `cargo embed` by:
//
//    > cargo install cargo-embed
//
//    or install from git:
//
//    >  cargo install --git https://github.com/probe-rs/cargo-embed.git
//
//    The host side handling of BlockIfFull was updated and merged 22h ago :)
//
// 1  Install the tool from git (as of now some features are not yet released).
//
//    Port your bare9 to the multi channel RTT.
//
//    See RTIC RTT fly!
//
//    > cargo embed --example rtic_bare9_no_block --release
//
//    It will start a gui for the RTT traces.
//    Use F1/F2 etc to switch between the channels.
//
//    `cargo-embed` is a part of the open source `probe.rs` project.
//
//    If you feel like making a better GUI, or logging to other
//    tools, e.g., parsing data to a graph or whatever, everything is possible.
//
//    Streams can be binary and you can use serde to transfer data this way.
//
//    It all depends on what you want, RTIC makes it possible.
