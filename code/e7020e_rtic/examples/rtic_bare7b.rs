// rtic_bare7b.rs
//
// What it covers
// - The embedded HAL
#![no_main]
#![no_std]

use panic_semihosting as _;

#[rtic::app(device = stm32f4::stm32f411, dispatchers = [EXTI0])]
mod app {
    use super::led;
    use cortex_m_semihosting::hprintln;
    use dwt_systick_monotonic::*;
    use stm32f4xx_hal::gpio::*;

    type LED = Pin<Output<PushPull>, 'A', 5>;

    // Default core clock at 16MHz
    const FREQ_CORE: u32 = 16_000_000;

    #[monotonic(binds = SysTick, default = true)]
    type MyMono = DwtSystick<FREQ_CORE>; // 16MHz cycle accurate accuracy

    #[shared]
    struct Shared {
        #[lock_free] // shared between tasks at the same priority
        led: LED,
    }

    #[local]
    struct Local {}

    #[init]
    fn init(cx: init::Context) -> (Shared, Local, init::Monotonics) {
        hprintln!("init").ok();

        let systick = cx.core.SYST;
        let mut dcb = cx.core.DCB;
        let dwt = cx.core.DWT;

        // Initialize the monotonic (SysTick driven by core clock)
        let mono = DwtSystick::new(&mut dcb, dwt, systick, FREQ_CORE);

        let gpioa = cx.device.GPIOA;

        let pa = gpioa.split();
        let led = pa.pa5.into_push_pull_output();

        led_on::spawn().ok();

        (Shared { led }, Local {}, init::Monotonics(mono))
    }

    #[idle]
    fn idle(_cx: idle::Context) -> ! {
        hprintln!("idle").ok();

        loop {}
    }

    #[task(shared = [led])]
    fn led_on(cx: led_on::Context) {
        hprintln!("rtic: led_on").ok();
        led::led_on(cx.shared.led);
        led_off::spawn_after(1.secs()).ok();
    }

    #[task(shared = [led])]
    fn led_off(cx: led_off::Context) {
        hprintln!("rtic: led_off").ok();
        led::led_off(cx.shared.led);
        led_on::spawn_after(1.secs()).ok();
    }
}

// This could be defined in a separated crate
// depending on embedded hal only.
mod led {
    use cortex_m_semihosting::hprintln;
    use embedded_hal::digital::v2::OutputPin;

    pub fn led_on<E>(led: &mut dyn OutputPin<Error = E>) {
        hprintln!("led_on").ok();
        // your code here
    }

    pub fn led_off<E>(led: &mut dyn OutputPin<Error = E>) {
        hprintln!("led_off").ok();
        // your code here
    }
}

// 1. Embedded Hal
//
//    Make sure you have the `embedded-hal` dependency in your `Cargo.toml`.
//
//    embedded-hal = "0.2.4"
//
//    Generate the documentation and lookup `digital::v2::OutputPin`.
//    You should find something like:
//
//    pub trait OutputPin {
//        type Error;
//        fn set_low(&mut self) -> Result<(), Self::Error>;
//        fn set_high(&mut self) -> Result<(), Self::Error>;
//
//        fn set_state(&mut self, state: PinState) -> Result<(), Self::Error> { ... }
//    }
//
//    The stm32fxx_hal implements the `embedded_hal::digital::v2::OutputPin` trait
//    for the OutputPin type:
//
//    Lookup the implementation (follow the [src] link)
//
//    What is the Error type for OutputPin.
//
//    ** your answer here **
//
// 2. Trait functions
//
//    Now look at the `mod led`, there you find two functions
//    `led_on` and `led_off`. Looking closer at the `led_on`:
//
//    pub fn led_on<E>(led: &mut dyn OutputPin<Error = E>) {...}
//
//    The `led` parameter is a trait object reference generic to the
//    Error type. (The concrete Error type is given by stm32fxx_hal as seen above).
//
//    You can operate on the `led` parameter through its API (the OutputPin trait).
//
//    Use the trait functions to make the led blink:
//    Hint: its super simple, just one line for `led_on`, one line for `led_off`.
//
//    Was it simple?
//
//    ** your answer here **
//
//    With this two lines of code you have written your first "library".
//    The `led` module is very portable, it depends ONLY the `embedded-hal`
//    (and not the `stm32f4xx-hal`), so will work on any hardware
//    which implements the `OutputPin` trait.
//
// 3. Efficiency
//
//    The `led` parameter is a trait object (reference). In general
//    trait objects are implemented by means of a VTABLE, and
//    executed by "dynamic dispatch". This results in an indirect
//    function call and hamper some possibilities for code optimization.
//
//    In many cases, Rust + LLVM can at compile time turn the
//    "dynamic dispatch" into a "static dispatch". There is however
//    no guarantee the compiler will succeed.
//
//    You can force static dispatch by using:
//    `&mut impl Trait` instead of
//    `&mut dyn Trait`.
//
//    Change the code to guarantee "static dispatch".
//
//    Was it simple?
//
//    ** your answer here **
//
//    With this two lines of code you have written your second "library",
//    now, with guaranteed "static dispatch".
//
//    In you static dispatch implementation you can even omit the type parameter,
//    and let the compiler infer the concrete Error type.
//
//    Remove the type parameter, and verify that it still works.
//
//    Was it simple?
//
//    ** your answer here **
//
// 4. Discussion
//
//    Rust aims at zero-cost abstractions, allowing you write code in a convenient
//    generic manner (for re-use and portability), without sacrificing performance.
//
//    Trait objects and "dynamic dispatch" are the ultimate tool for flexibility,
//    while "static dispatch" is less flexible but give guaranteed performance.
//
//    Where are trait objects actually needed, you may ask?
//
//    Say that we have two completely different Output pins in our application.
//    A) an stm32f4 gpio pin (which never fail, Error = ())
//    B) a pin behind some IO extender (which might fail at run-time).
//
//    If we want to build a collection (e.g., list or array) of Output pins A and B,
//    then you would need a to use trait object, in all other cases
//    trait implementations are preferable.
