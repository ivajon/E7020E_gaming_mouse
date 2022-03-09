//! bare1.rs
//!
//! Inspecting the generated assembly in Vscode.
//!
//! What it covers
//! - Rust panic on arithmetics
//! - assembly calls and inline assembly

#![no_main]
#![no_std]

use panic_semihosting as _;

#[rtic::app(device = stm32f4::stm32f401)]
mod app {
    use cortex_m_semihosting::hprintln;

    #[shared]
    struct Shared {}

    #[local]
    struct Local {}

    // do nothing in init
    #[init]
    fn init(_: init::Context) -> (Shared, Local, init::Monotonics) {
        (Shared {}, Local {}, init::Monotonics())
    }

    // idle never returns, function has the "never type" !
    #[idle]
    fn idle(_: idle::Context) -> ! {
        hprintln!("hello").ok();
        let mut x = core::u32::MAX - 1;
        loop {
            // cortex_m::asm::bkpt();
            x += 1;
            // cortex_m::asm::bkpt();

            // prevent optimization by read-volatile (unsafe)
            unsafe {
                core::ptr::read_volatile(&x);
            }
        }
    }
}

// 0. Setup
//    Make sure that your repository is updated (pull from upstream).
//
// 1. Click on the [Run and Debug] button (or press Ctrl-Shift-D)
//    Choose the "Cortex Debug (No ITM)" profile in the dropdown.
//    Click on the Start Debugging arrow (or press F5).
//
//    This compiles the chosen example in dev/debug mode and
//    and starts a debug session (openocd/gdb will be started).
//    The program will be flashed, started and halted on "main".
//    (RTIC will generate a "main" for you, that calls "init" etc.)
//
//    You should see the "#[rtic::app(device = stm32f4::stm32f411)]"
//    highlighted (the MCU is halted at "main" before "init").
//
//    You can now press the "Continue" arrow (or press F5).
//    This causes the program to continue, in this case it will crash.
//
//    Now select OUTPUT and [gdb-server].
//
//    You should have encountered a Rust panic.
//
//    Paste the error message:
//
//    ** your answer here **
//
//    Look at the source code and Explain in your own words why the code panic:ed.
//
//    ** your answer here **
//
//    Commit your answer (bare1_1)
//
// 2. Inspecting what caused the panic.
//
//    Under CALL STACK *to the left) you find the calls done to reach the panic:
//
//    You can also get the same information directly from GDB
//
//    Select the DEBUG CONSOLE (Ctrl-Shift Y) and enter the command
//
//    > backtrace
//
//    Paste the backtrace:
//
//    ** your answer here
//
//    Explain in your own words the chain of calls.
//
//    ** your answer here
//
//    Commit your answer (bare1_2)
//
// 3. Now let's try to break it down to see what caused the panic.
//
//    Put a breakpoint at line 37 (x += 1;)
//    (Click to the left of the line marker, you get a red dot.)
//
//    Restart the debug session (Ctrl-Shift F5), and continue until you hit the breakpoint.
//
//    What is the value of `x`?
//
//    ** your answer here **
//
//    Explain in your own words where this value comes from.
//
//    ** your answer here **
//
//    Now continue the program (F5), since you are in a loop
//    the program will halt again at line 37.
//
//    What is the value of `x`?
//
//    Explain in your own words why `x` now has this value.
//
//    ** your answer here **
//
//    Now continue again.
//
//    At this point your code should panic.
//
//    You can navigate the CALL STACK.
//    Click on rtic_bare::idle@0x08.. (37)
//
//    The line leading up to the panic should now be highlighted.
//    So you can locate the precise line which caused the error.
//
//    Explain in your own words why a panic makes sense at this point.
//
//    ** your answer here **
//
//    Commit your answer (bare1_3)
//
// 4. Now lets have a look at the generated assembly.
//
//    First restart the debug session and continue to the first halt (line 37).
//
//    Select DEBUG CONSOLE and give the command
//
//    > disassemble
//
//    The current PC (program counter is marked with an arrow)
//    => 0x08... <+38>:	ldr	r0, [sp, #0]
//
//    Explain in your own words what this assembly line does.
//
//    ** your answer here **
//
//    In Cortex Registers (left) you can see the content of `r0`
//
//    What value do you observe?
//
//    ** your answer here **
//
//    You can also get the register content from GDB directly.
//
//    > info register
//
//    Many GDB commands have short names try `i r`.
//
//    So now, time to move on, one assembly instruction at a time.
//
//    > stepi
//    > disassemble
//
//    Now you should get
//    => 0x08... <+40>:	adds	r0, #1
//
//    Explain in your own words what is happening here.
//
//    ** your answer here **
//
//    We move to the next assembly instruction:
//
//    > si
//    > i r
//
//    What is the reported value for `r0`
//
//    ** your answer here **
//
//    So far so good.
//
//    We can now continue to the next breakpoint.
//
//    > continue
//    (or in short >c, or press F5, many options here ...)
//    > disassemble
//    (or in short >disass)
//    you may also navigate prior input using Up/Down arrows)
//
//    You should now be back at the top of the loop:
//
//    => 0x08... <+38>:	ldr	r0, [sp, #0]
//
//    and the value of `r0` should be -1 (or 0xffffffff in hexadecimal)
//
//    Now we can step an instruction again.
//
//    > si
//    => 0x08... <+40>:	adds	r0, #1
//
//    So far so good, and another one.
//
//    > si
//    => bcs.n	0x8... <rtic_bare1::app::idle+54>
//
//    lookup the arm instruction set: https://developer.arm.com/documentation/ddi0210/c/Introduction/Instruction-set-summary/Thumb-instruction-summary
//
//    What does BCS do?
//
//    ** your answer here **
//
//    Now let's see what happens.
//
//    > si
//    > disass
//    => 0x08... <+54>:	movw	r0, #6832	; 0x1ab0
//       0x08... <+58>:	movw	r2, #6804	; 0x1a94
//       0x08... <+62>:	movt	r0, #2048	; 0x800
//       0x08... <+66>:	movt	r2, #2048	; 0x800
//       0x08... <+70>:	movs	r1, #28
//       0x08... <+72>:	bl	0x8000308 <core::panicking::panic>
//
//    Explain in your own words where we are heading.
//
//    ** your answer here **
//
//    To validate that your answer, let's let the program continue
//
//    > c
//
//    Look in the OUTPUT/Adapter Output console again.
//
//    Explain in your own words what the code
//    0x08...<+54> ..  0x08..<+72> achieves
//
//    Hint 1, look at the error message?
//    Hint 2, look at the call stack.
//    Hint 3, the code is generated by the Rust compiler to produce the error message.
//            there is no "magic" here, just a compiler generating code...
//
//    ** your answer here **
//
//    Commit your answer (bare1_4)
//
// 5. Now we can remove the break point (click the `Remove All Breakpoints`),
//    and instead uncomment the two breakpoint instructions (on lines 36 and 38).
//
//    Close the debug session and press F5 again to re-compile and launch the app.
//
//    Continue until you hit the first breakpoint.
//
//    The disassembly should look like this:
//    (Notice, actual addresses might differ)
//
//
//    0x08000e44 <+38>:	bl	0x8001746 <lib::__bkpt>
// => 0x08000e48 <+42>:	ldr	r0, [sp, #0]
//    0x08000e4a <+44>:	adds	r0, #1
//    0x08000e4c <+46>:	bcs.n	0x8000e5c <rtic_bare1::app::idle+62>
//    0x08000e4e <+48>:	str	r0, [sp, #0]
//    0x08000e50 <+50>:	bl	0x8001746 <lib::__bkpt>
//    0x08000e54 <+54>:	mov	r0, r4
//    0x08000e56 <+56>:	bl	0x80016f8 <core::ptr::read_volatile<u32>>
//    0x08000e5a <+60>:	b.n	0x8000e44 <rtic_bare1::app::idle+38>
//
//    In stable Rust we cannot currently write inline assembly, thus we do a "workaround"
//    and call a function that that contains the assembly instruction.
//
//    In this code:
//       0x08000e44 <+38>:	bl	0x8001746 <lib::__bkpt>
//    and
//       0x08000e50 <+50>:	bl	0x8001746 <lib::__bkpt>
//
//    In cases, this is not good enough (if we want exact cycle by cycle control).
//    We can overcome this by letting the linker inline the code.
//
//    Let's try this, build and run the code in release mode (Cortex Release (No ITM)).
//    Continue until you hit the first assembly breakpoint.
//
//    The disassembly now should look like this:
//
//       0x08000296 <+154>:	mvn.w	r0, #1
//       0x0800029a <+158>:	adds	r0, #1
//    => 0x0800029c <+160>:	bkpt	0x0000
//       0x0800029e <+162>:	str	r0, [sp, #4]
//       0x080002a0 <+164>:	bkpt	0x0000
//       0x080002a2 <+166>:	ldr	r0, [sp, #4]
//       0x080002a4 <+168>:	b.n	0x800029a <rtic_bare1::app::idle+158>
//
//    Now let's compare the two assembly snippets.
//    We now see that the breakpoints have been inlined (offsets +160, +164).
//
//    But something else also happened here!
//
//    Do you see any way this code may end up in a panic?
//
//    ** your answer here **
//
//    So clearly, the "semantics" (meaning) of the program has changed.
//    This is on purpose, Rust adopts "unchecked" (wrapping) additions (and subtractions)
//    by default in release mode (to improve performance).
//
//    The downside, is that programs change meaning. If you intend the operation
//    to be wrapping you can explicitly express that in the code.
//
//    Change the `x += 1` to `x = x.wrapping_add(1)`.
//
//    And recompile/run/the code in Debug mode (Cortex Debug (No ITM))
//
//    Paste the generated assembly:
//
//    ** your answer here **
//
//    Can this code generate a panic?
//
//    ** your answer here **
//
//    Is there now any reference to the panic handler?
//    If not, why is that the case?
//
//    ** your answer here **
//
//    commit your answers (bare1_5)
//
//    Now, change back to
//    "x = x.wrapping_add(1)" back to "x += 1"
//
//    Discussion:
//    In release (optimized) mode the addition is unchecked,
//    so there is a semantic difference here in between
//    the dev and release modes. This is motivated by:
//    1) efficiency, unchecked/wrapping is faster
//    2) convenience, it would be inconvenient to explicitly use
//    wrapping arithmetics, and wrapping is what the programmer
//    typically would expect in any case. So the check
//    in dev/debug mode is just there for some extra safety
//    if your intention is NON-wrapping arithmetics.
//
//    The debug build should have additional code that checks if the addition
//    wraps (and in such case call panic). In the case of the optimized
//    build there should be no reference to the panic handler in the generated
//    binary. Recovering from a panic is in general very hard. Typically
//    the best we can do is to stop and report the error (and maybe restart).
//
// 6. Now comment out the "read_volatile".
//
//    Rebuild and run the code in Release mode (Cortex Release (No ITM)).
//
//    Dump the generated assembly.
//
//    ** your answer here **
//
//    Where is the local variable stored?
//    What happened, and why is Rust + LLVM allowed to optimize out your code?
//
//    ** your answer here **
//
//    Commit your answers (bare1_6)
//
//    Now, un-comment "read_volatile" as it was before.
//
// 7. If you always want wrapping arithmetics (also in dev/debug) mode,
//    you can uncomment `# overflow-checks = false` in the Cargo.toml file.
//
//    What is now the disassembly of the loop (in debug/dev mode):
//
//    ** your answer here **
//
//    commit your answers (bare1_7)
//
//    Final discussion:
//
//    Embedded code typically is performance sensitive, hence
//    it is important to understand how code is generated
//    to achieve efficient implementations.
//
//    Moreover, arithmetics are key to the processing of data,
//    so its important that we are in control over the computations.
//    E.g. computing checksums, hashes, cryptos etc., all
//    require precise control over wrapping vs. overflow behavior.
//
//    If you write a library depending on (wrapping) arithmetics
//    make sure the code works both in debug/dev and release mode.
//
//    Why is this important?
//
//    ** your answer here **
//
//    commit your answers (bare1_8)
//
//    You may return to this exercise to recap how you can do fine
//    grained debugging if run into problems later.
//
