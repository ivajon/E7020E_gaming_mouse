//! rtic_bare3.rs
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
    use fugit::MillisDuration;
    use systick_monotonic::*;

    // Default core clock at 16MHz
    const FREQ_CORE: u32 = 16_000_000;

    #[monotonic(binds = SysTick, default = true)]
    type MyMono = Systick<100>; // 100 Hz / 10 ms granularity

    #[shared]
    struct Shared {}

    #[local]
    struct Local {}

    #[init]
    fn init(cx: init::Context) -> (Shared, Local, init::Monotonics) {
        let systick = cx.core.SYST;

        // Initialize the monotonic (SysTick driven by core clock)
        let mono = Systick::new(systick, FREQ_CORE);

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
        let millis: MillisDuration<u64> = duration.convert();

        // Formatting using the Debug trait
        hprintln!("Diff {:?}", millis).ok();
        // Formatting using the Display trait
        hprintln!("Diff {}", millis).ok();
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

// 0. Setup
//
//    > cargo doc --example rtic_bare3 --open
//    (When documenting make sure that only one panic handler is enabled in Cargo.toml.)
//
//    In the docs, search (`S`) for `Monotonic` and read the API docs.
//    Also search for `Instant`, and `Duration`.
//
//    Together these provide timing semantics.
//
//    - `Monotonic` is a "trait" for a timer implementation.
//    - `Instant` is a point in time.
//    - `Duration` is a range in time.
//
// 1. Build and run the application in vscode using (Cortex Debug).
//
//    a) What is the output in the Terminal [gdb-server]?
//
//    ** your answer here **
//
//    Now, build and run the application in vscode using (Cortex Release).
//
//    b) What is the output in the Terminal [gdb-server]?
//
//    ** your answer here **
//
//    Compute the speedup (how many times faster is release)
//
//    ** your answer here **
//
//    Commit your answers (bare3_1)
//
// 2. The implementation of Systick as a monotonic timer sets up periodic interrupt,
//    in this case with a period of 10 milliseconds.
//
//    On each interrupt the monotonic timer is increased.
//
//    Change the timer granularity to 10 micro seconds.
//
//    Build and run the application in vscode using (Cortex Release)
//
//    What is the output in the Terminal [gdb-server]
//
//    ** your answer here **
//
//    Explain in your own words why the measured values differs from bare3_1b.
//
//    ** your answer here **
//
//    Commit your answers (bare3_2)
//
// 4. Discussion.
//
//    In this exercise we have shown that we can step away from pure hardware accesses
//    and deal with time in a more convenient and "abstract" fashion.
//
//    `Instant` and `Duration` are associated with semantics (meaning).
//    `Monotonic` as a trait is associated with the API and its implementation.
//
//    This is an example of separation of concerns!
//
//    If you implement your application based on Instant and Duration, your code
//    will be "portable" across all platforms (that implement the chosen Monotonic).
//
//    The implementation of a Monotonic timer is done only once for each platform, thus
//    bugs related to low level timer access will occur only at one place,
//    not scattered across thousands of manually written applications.
//
//    However, when using the Systick to make a "tick based" monotonic timer the granularity
//    will severely affect the overhead and real-time performance of your system.
//
//    As a comparison, the Linux kernel is driven by a system tick (jiffy).
//    By default it is set to 10ms, as a reasonable tradeoff between timing accuracy and
//    interrupt handling overhead.
