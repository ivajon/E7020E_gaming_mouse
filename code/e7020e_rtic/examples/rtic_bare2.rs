//! rtic_bare2.rs
//!
//! Measuring execution time
//!
//! What it covers
//! - Generating documentation
//! - Using core peripherals
//! - Measuring time using the DWT

#![no_main]
#![no_std]

use panic_semihosting as _;

#[rtic::app(device = stm32f4::stm32f411)]
mod app {
    use super::wait;
    use cortex_m::peripheral::DWT;
    use cortex_m_semihosting::hprintln;

    #[shared]
    struct Shared {}

    #[local]
    struct Local {}

    #[init]
    fn init(mut ctx: init::Context) -> (Shared, Local, init::Monotonics) {
        ctx.core.DWT.enable_cycle_counter();

        // Reading the cycle counter can be done without `owning` access
        // the DWT (since it has no side effect).
        //
        // Look in the docs:
        // pub fn enable_cycle_counter(&mut self)
        // pub fn cycle_count() -> u32
        //
        // Notice the difference in the function signature!

        let start = DWT::cycle_count();
        wait(1_000_000);
        let end = DWT::cycle_count();

        // notice all printing outside of the section to measure!
        hprintln!("Start {:?}", start).ok();
        hprintln!("End {:?}", end).ok();
        hprintln!("Diff {:?}", end.wrapping_sub(start)).ok();

        // wait(100);
        (Shared {}, Local {}, init::Monotonics())
    }
}

// burns CPU cycles by just looping `i` times
#[inline(never)] // forbids inlining at call site
#[no_mangle] // identifier in human readable form
pub fn wait(i: u32) {
    for _ in 0..i {
        // no operation (ensured not optimized out)
        cortex_m::asm::nop();
    }
}

// 0. Setup
//
//    > cargo doc --example rtic_bare2 --open
//
//    `cargo.doc` will document your crate, and open the docs in your browser.
//    If it does not auto-open, then copy paste the path shown in your browser.
//
//    Notice, it will try to document all dependencies, you may have only one
//    one panic handler, so temporarily comment out all but one in `Cargo.toml`.
//
//    In the docs, search (`S`) for DWT, and click `cortex_m::peripheral::DWT`.
//    Read the API docs.
//
// 1. Build and run the application in vscode using (Cortex Debug).
//
//    What is the output in the Adapter Output console?
//    (Notice, it will take a while we loop one million times at only 16 MHz.)
//
//    ** your answer here **
//
//    Rebuild and run in (Cortex Release).
//
//    ** your answer here **
//
//    Compute the ratio between debug/release optimized code
//    (the speedup).
//
//    ** your answer here **
//
//    commit your answers (bare2_1)
//
// 2. As seen there is a HUGE difference in between Debug and Release builds.
//    In Debug builds, the compiler preserves all abstractions, so there will
//    be a lot of calls and pointer indirections.
//
//    In Release builds, the compiler strives to "smash" all abstractions into straight
//    line code.
//
//    This is what Rust "zero-cost abstractions" means, not zero execution time but rather,
//    "as good as it possibly gets" (you typically pay no extra cost for using abstractions at run-time).
//
//    In Release builds, the compiler is able to "specialize" the implementation
//    of each function.
//
//    Let us look in detail at the "wait" function:
//    Place a breakpoint at line 60 (nop in wait).
//
//    Restart the (Cortex Release) session and look at the generated code.
//
//    > disass
//
//    Dump generated assembly for the "wait" function.
//
//    ** your answer here **
//
//    Under the ARM calling convention, r0, r1, ... is used as arguments.
//    In this case, however we se that r0 is set in "wait".
//
//    Explain in your own words why this Ok (in this case)
//
//    ** your answer here **
//
//    Lookup the two instructions `movw` and `movt` to figure out what happens here.
//
//    Answer in your own words, how they assign r0 to 1000000.
//
//    ** your answer here **
//
//    Commit your answers (bare2_2)
//
// 3. Now add a second call to `wait` (line 49).
//
//    Recompile and run until the breakpoint.
//
//    Dump the generated assembly for the "wait" function.
//
//    ** your answer here **
//
//    Answer in your own words, why you believe the generated code differs to bare_2_2?
//
//    ** your answer here **
//
//    Commit your answers (bare2_3)
//
// 4. Now add a breakpoints at line 41 and 49, (wait calls).
//
//    Run the code in release mode till you hit the the breakpoint at line 41.
//    Disassemble the code and look for the current PC (=>)
//    (you will need to scroll up a bit)
//
//    What assembly instructions related to "r0" precede the "bl 0x8... <rtic_bare2::wait>"
//
//    ** your answer here **
//
//    Now run the code till the breakpoint at line 49.
//
//    Disassemble the code and look for the current PC (=>)
//
//    What assembly instructions related to "r0" precede the "bl 0x8... <rtic_bare2::wait>"
//
//    ** your answer here **
//
//    Explain in your own words what the preceding code before the "bl" in both cases.
//
//    ** your answer here **
//
//    Discussion:
//    Whe have in this example seen that optimization is very important
//    to obtain efficient executables (huge difference between debug and release builds).
//
//    We have also seen that aggressive optimization done by the compiler
//    may promote argument values into functions to improve performance.
//    (In this case the performance was likely the same, but overall value
//    promotion can lead to further optimizations and thus better performance.)
