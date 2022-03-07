// rtic_bare7.rs
//
// What it covers
// - Reading Rust documentation
// - Using the HAL abstractions

#![no_main]
#![no_std]

use panic_semihosting as _;

#[rtic::app(device = stm32f4::stm32f411, dispatchers = [EXTI0])]
mod app {
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
        hprintln!("led_on").ok();
        cx.shared.led.set_high();
        led_off::spawn_after(1.secs()).ok();
    }

    #[task(shared = [led])]
    fn led_off(cx: led_off::Context) {
        hprintln!("led_off").ok();
        cx.shared.led.set_low();
        led_on::spawn_after(1.secs()).ok();
    }
}

// 1. Run it and make sure you have understood how the application works.
//
//    In particular look at the usage of Monotonic to spawn tasks in the future,
//    and how we can get lock free access to the shared `led` resource.
//
//    Now, let's have a look at the HAL for stm32f4
//    https://github.com/stm32-rs/stm32f4xx-hal
//
//    You need stm32f4xx-hal as a dependency in your Cargo.toml:
//
//    [dependencies.stm32f4xx-hal]
//    version = "0.11.1"
//    features = ["rt", "stm32f411"]
//
//    Now, document the crate to have access to the latest dependencies.
//
//    > cargo doc --example rtic_bare6 --open
//
//    Now open `rtic_blinky` side by side so you can compare `rtic_blinky` and
//    and the code from `rtic_bare7`.
//
//    If you have a large screen you can open `rtic_blinky_hal` side by side,
//    to view all three files at the same time.
//    (The code in this file is the same as in `rtic_blinky_hal`.)
//
//    Now we will compare `rtic_blinky` to `rtic_blink_hal`, in detail.
//
//    In `rtic_blink` you will find:
//
//     #[shared]
//     struct Shared {
//         #[lock_free] // shared between tasks at the same priority
//         gpioa: stm32f4::stm32f411::GPIOA,
//     }
//
//    In `rtic_blink_hal` you will find:
//
//    #[shared]
//    struct Shared {
//         #[lock_free] // shared between tasks at the same priority
//         led: LED,
//    }
//
//    What is the type of LED?
//
//    ** your answer here **
//
//    Now look at the `rtt_blinky_hal`
//
//    let pa = gpioa.split();
//
//    What is the type of the `pa` variable?
//    Hint, Rust analyzer will infer the type:
//
//    ** your answer here **
//
//    What is happening here is that the HAL splits the
//    `gpioa` into its individual parts allowing you to
//    operate on individual pins of the gpio port.
//
//    What is the type of the `led` variable.
//    Hint, Rust analyzer will infer the type:
//
//    ** your answer here **
//
//    Rust analyzer will not show the `const` type parameters,
//    since Rust does not yet support `const` generic inference.
//
//    Compare the `init` task of `rtic_blinky` and `rtic_blinky_hal`.
//
//    Where do you think the enabling of GPIOA in RCC is done in the `hal` version.
//
//    ** your answer here **
//
//    Where is the pin mode set in in the `hal` code.
//
//    ** your answer here **
//
//    Now we can have a look at the business logic.
//
//    Compare the `led_on` tasks in `rtic_blinky` and `rtic_blinky_hal`.
//
//    In `rtic_blinky` you find:
//
//    cx.shared.gpioa.bsrr.write(|w| w.br5().set_bit());
//
//    In `rtic_blinky_hal` you find:
//
//    cx.shared.led.set_high();
//
//    What do you think is better and why?
//
//    ** your answer here **
//
// 2. Misconfiguration
//
//    Now change the code in `rtic_blinky`.
//
//    Replace:
//
//    gpioa.moder.write(|w| w.moder5().output());
//
//    By:
//
//    gpioa.moder.write(|w| w.moder5().input());
//
//    Run the example: Why did it not work?
//
//    ** your answer here **
//
//    (Revert the change)
//
//    Now change the code in `rtic_blinky_hal`
//
//    Replace:
//
//    let led = pa.pa5.into_push_pull_output();
//
//    By:
//
//    let led = pa.pa5.into_floating_input();
//
//    Run the example: Why did it not work?
//
//    ** your answer here **
//
//    Now revisit the question:
//
//    What do you think is better and why?
//
//    (Revert the change)
//
//    ** your answer here **
//
// 3. Carrying too specific information?
//
//    The concrete type of the LED resource is
//
//    Pin<Output<PushPull>, 'A', 5>;
//
//    But from the usage point of view the
//    port and pin number is irrelevant.
//
//    Replace the type definition of LED:
//
//    type LED = Pin<Output<PushPull>, 'A', 5>;
//
//    By:
//
//    type LED = ErasedPin<Output<PushPull>>;
//
//    The `ErasedPin<MODE>` represents a pin without
//    any specific port or pin number, thus more generic.
//
//    Look in the Rust documentation for `erase`, and
//    use that to construct the `led`.
//
//    What is now the type of the `led`.
//    Hint, Rust analyser will infer the type for you.
//
//    ** your answer here **
//
// 4. Discussion:
//
//    Here we have seen the Rust ecosystem excel!
//
//    The HAL uses types for representing the state of the underlying hardware.
//    This allows to catch configurations errors and other
//    types of mis-use at compile time. We can operate on the state
//    and get results in terms of a new type representing the new state.
//
//    As you have seen, without type states we could introduce
//    errors without knowing. At best we catch these errors
//    when testing (we saw the led was not blinking after
//    mis-configuring the pin), but in general you will never
//    know if you have tested enough to ensure correctness.
//
//    You don't really want to send a lander to Mars just to realize
//    there was a bug that caused it to loos communication, likewise
//    you don't want to recall 5 million Toyotas due to a bug
//    in the anti lock breaks:
//
//    https://en.wikipedia.org/wiki/2009%E2%80%932011_Toyota_vehicle_recalls
//
//    Now revisit the question:
//
//    What do you think is better and why?
//
//    ** your answer here **
//
//    Rust + RTIC + HAL is it perfect?
//
//    No, its not ... yet. There are many things to improve. But its a huge
//    step towards more reliable (or less un-reliable) embedded software.
//
//    E.g., the HAL design:
//
//    We can easily
//
//    // implicitly enable GPIO;
//    let pa = gpioa.split();
//
//    // some code that (by mistake or attack) disables gpioa
//    rcc.ahb1enr.write(|w| w.gpioaen().disabled());
//
//    ... code that uses the `pa`
//
//    This can be solved by a `constrain/freeze` pattern
//    (e.g., look at how the clock abstraction uses this).
//
//   YOU can make a change!
//
//   Spot shortcomings in RTIC and the embedded Rust ecosystem.
//   File issues, make PRs, and we will land an Mars!!!
