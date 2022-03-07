//! rtic_bare3b.rs
//!
//! Measuring execution time
//!
//! What it covers
//! - Reading Rust documentation
//! - Timing abstractions and semantics
//! - Understanding Rust abstractions

#![no_main]
#![no_std]

use panic_halt as _;

#[rtic::app(device = stm32f4::stm32f411)]
mod app {
    use super::wait;
    use cortex_m_semihosting::hprintln;
    use dwt_systick_monotonic::*;
    use fugit::{MicrosDuration, MillisDuration};

    // Default core clock at 16MHz
    const FREQ_CORE: u32 = 16_000_000;

    #[monotonic(binds = SysTick, default = true)]
    type MyMono = DwtSystick<FREQ_CORE>; // 16MHz cycle accurate accuracy

    #[shared]
    struct Shared {}

    #[local]
    struct Local {}

    #[init]
    fn init(cx: init::Context) -> (Shared, Local, init::Monotonics) {
        let systick = cx.core.SYST;
        let mut dcb = cx.core.DCB;
        let dwt = cx.core.DWT;

        // Initialize the monotonic (SysTick driven by core clock)
        let mono = DwtSystick::new(&mut dcb, dwt, systick, FREQ_CORE);

        (Shared {}, Local {}, init::Monotonics(mono))
    }

    #[idle]
    fn idle(_cx: idle::Context) -> ! {
        let start = monotonics::now();
        wait(1_000_000);
        let end = monotonics::now();

        // notice all printing outside of the section to measure!
        hprintln!("Start {:?}", start).ok();
        hprintln!("End {:?}", end).ok();
        let duration = end - start;
        hprintln!("Diff {:?}", duration).ok();

        let millis: MillisDuration<u32> = duration.convert();

        // Formatting using the Debug trait
        hprintln!("Diff {:?}", millis).ok();
        // Formatting using the Display trait
        hprintln!("Diff {}", millis).ok();

        let micros: MicrosDuration<u32> = duration.convert();

        // Formatting using the Debug trait
        hprintln!("Diff {:?}", micros).ok();
        // Formatting using the Display trait
        hprintln!("Diff {}", micros).ok();
        loop {}
    }
}

// burns CPU cycles by just looping `i` times
#[inline(never)]
#[no_mangle]
fn wait(i: u32) {
    for _ in 0..i {
        // no operation (ensured not optimized out)
        cortex_m::asm::nop();
    }
}

// 1. In this example we use `DwtSystick` as a Monotonic timer.
//    The DWT is used as free running clock (as a monotonic time base),
//    while the Systick is used only for scheduling timed tasks.
//
//    As we don't (yet) schedule any timed tasks there won't be any Systick interrupts.
//
//    Run the example in vscode (Cortex Release).
//
//    What is the output in the Terminal [gdb-server]?
//
//    ** your answer here **
//
//    Compare the output to the measurements of `bare2` and `bare3`.
//
//    What do you find?
//
//    ** your answer here **
//
// 2. Discussion.
//
//    In this exercise we have shown that it is possible to obtain high precision
//    timing granularity almost without performance regression.
//
//    At the same time we can reason on timing measurements using abstractions,
//    and make code portable across platforms.
//
//    The Monotonic trait just defines the API for a monotonic timer.
//    E.g., we can have a Real-Time clock (with very low drift) for calendar like
//    functionality, and at the same time another monotonic timer that
//    provide fine/high granularity (but perhaps shorter range.)
//
//    RTIC supports applications with arbitrary number of Monotonic time implementations.
