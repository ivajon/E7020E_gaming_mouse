//! rtic_bare6.rs
//!
//! Measuring execution time
//!
//! What it covers
//! - Reading Rust documentation
//! - Timing abstractions and semantics
//! - Understanding Rust abstractions

#![no_main]
#![no_std]

use panic_semihosting as _;

#[rtic::app(device = stm32f4::stm32f411)]
mod app {
    use super::{clock_out, wait};
    use cortex_m_semihosting::hprintln;
    use dwt_systick_monotonic::*;
    use fugit::{MicrosDuration, MillisDuration};
    use stm32f4xx_hal::prelude::*;

    // Core clock at 48MHz
    const FREQ_CORE: u32 = 48_000_000;

    #[monotonic(binds = SysTick, default = true)]
    type MyMono = DwtSystick<FREQ_CORE>; // 48MHz cycle accurate accuracy

    #[shared]
    struct Shared {}

    #[local]
    struct Local {}

    #[init]
    fn init(cx: init::Context) -> (Shared, Local, init::Monotonics) {
        let systick = cx.core.SYST;
        let mut dcb = cx.core.DCB;
        let dwt = cx.core.DWT;
        let rcc = cx.device.RCC;

        // rout the clock to output port PC9
        clock_out(&rcc, &cx.device.GPIOC);

        // set core clock to 48MHz and freeze the settings
        let clk = rcc.constrain().cfgr.sysclk(48.MHz()).freeze();

        // Initialize the monotonic (SysTick driven by core clock)
        let mono = DwtSystick::new(&mut dcb, dwt, systick, clk.sysclk().raw());

        // let clk = rcc.constrain().cfgr.sysclk(64.mhz()).freeze();

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

// see the Reference Manual RM0368 (www.st.com/resource/en/reference_manual/dm00096844.pdf)
// rcc,     chapter 6
// gpio,    chapter 8

use stm32f4::stm32f411::{GPIOC, RCC};

fn clock_out(rcc: &RCC, gpioc: &GPIOC) {
    // output MCO2 to pin PC9

    // mco2 	: SYSCLK = 0b00
    // mcopre 	: divide by 4 = 0b110
    rcc.cfgr
        .modify(|_, w| unsafe { w.mco2().bits(0b00).mco2pre().bits(0b110) });

    // power on GPIOC, RM0368 6.3.11
    rcc.ahb1enr.modify(|_, w| w.gpiocen().set_bit());

    // MCO_2 alternate function AF0, STM32F401xD STM32F401xE data sheet
    // table 9
    // AF0, gpioc reset value = AF0

    // configure PC9 as alternate function 0b10, RM0368 6.2.10
    gpioc.moder.modify(|_, w| w.moder9().bits(0b10));

    // otyper reset state push/pull, in reset state (don't need to change)

    // ospeedr 0b11 = very high speed
    gpioc.ospeedr.modify(|_, w| w.ospeedr9().bits(0b11));
}

// 1. Run the example in vscode (Cortex Release).
//
//    What is the output in the Terminal [gdb-server]?
//
//    ** your answer here **
//
//    Compare the output to the measurements of `bare2` and `bare3`.
//
//    What do you find, explain the speedup?
//
//    ** your answer here **
//
// 2. Now look at the documentation of the Nucleo board and find PC9
//
//    Connect the pin to an oscilloscope and measure the frequency.
//
//    What frequency did you observe?
//
//    ** your answer here **
//
//    Why did you observe the this frequency (and not 48MHz)?
//
//    Hint, look at the `clock_out` code.
//
//    ** your answer here **
//
// 3. Generate documentation for your crate (if dependencies changed).
//
//    Lookup the Rust documentation for `mco2pre2`. Compare that to the documentation
//    in the Reference manual for `mco2`.
//
//    Change the code in `clock_out` to use the function API instead of the row bits `0b110`.
//
//    Re-run the measurement to confirm that you observe the same frequency.
//
//    Change the divider, to `div2`.
//
//    Re-run the measurement.
//
//    What frequency did you now observe?
//
//    ** your answer here **
//
//    Why do you think there is a divide setting for the `mco2`?
//
//    ** your answer here **
//
//    Now lookup the `mco2`, `moder9` and `opseedr9` Rust documentation and change
//    the code in `clock_out` to use the function API.
//
//    Re-run the measurement to verify that you observe the same frequency.
//
// 4. Now you can try mis-configuring the clock.
//
//    Our assumption is that the core clock should run at 48MHz. (line 46)
//
//    Let's try to reconfigure the clock after we have set the monotonic.
//
//    Uncomment line 51
//
//    let clk = rcc.constrain().cfgr.sysclk(64.mhz()).freeze();
//
//    Now try to compile and run the project:
//
//    What happens?
//
//    ** your answer here **
//
//    As seen the framework also saves us from making such mistakes which is
//    great, the RCC is consumed by the first configuration and cannot be reconfigured
//    by mistake or malicious code.
//
//    Now let's try another misconfiguration (line 46)
//
//    Replace:
//
//    let clk = rcc.constrain().cfgr.sysclk(48.mhz()).freeze();
//
//    With:
//
//    let clk = rcc.constrain().cfgr.sysclk(64.mhz()).freeze();
//
//    Re-run the code:
//
//    What is the output in the semihosting log:
//
//    ** your answer here **
//
//    Why did we get this error?
//
//    ** your answer here **
//
//    As seen the framework also saves us here from running a mis-configured system.
//    However it is not bullet proof, (I made an error in an earlier commit, that snuck through).
//    Unfortunately, as of now the Rust type system is not powerful enough to reject this
//    type of errors at compile time, so the best we can do currently is a run-time check.
//
// 5. Discussion:
//    In this exercise you have seen how the clock frequency affects the execution speed.
//
//    You have also confirmed the (core) clock frequency was correctly set
//    by measuring the MCO2.
//
//    Moreover you have seen that the function API is useful to clarify the intent
//    of low-level code and makes it more or less self documenting.
//
//    Finally, we have seen how the framework abstractions saves us from running
//    misconfigured applications.
