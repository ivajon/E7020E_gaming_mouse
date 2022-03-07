// From terminal:
// cargo run --example --release rtic_crash
// Assumes cargo/config.toml
// runner = "arm-none-eabi-gdb -q -x openocd.gdb"
// Assumes `openocd` running in separate terminal
// > openocd -f openocd.cfg
//
// From vscode:
// Select (Cortex Release (No ITM))
// Press F5 (to compile/flash/debug the application)
// DEBUG CONSOLE (to view/control the gdb session)

#![no_main]
#![no_std]

use panic_halt as _;
#[rtic::app(device = stm32f4::stm32f411)]
mod app {
    use core::ptr;

    #[shared]
    struct Shared {}

    #[local]
    struct Local {}

    #[init]
    fn init(_: init::Context) -> (Shared, Local, init::Monotonics) {
        unsafe {
            // read an address outside of the RAM region; this causes a HardFault exception
            ptr::read_volatile(0x2FFF_FFFF as *const u32);
        }

        (Shared {}, Local {}, init::Monotonics())
    }
}

// Here you can inspect the call stack (found to the left in vscode).
//
// The default implementation for `HardFault` exception is just an infinite loop.
// Press the pause symbol to halt the processor:
//
// The upmost item in CALL STACK, is the current frame:
// (the infinite loop)
//
// The bottom most item is the start of the program (the generated main).
//
// In between, you can see the calls made
// main->init->read_volatile->HardFault->compiler_fence
//
// Click on init, and you will see that line 14 in this application caused the
// erroneous read operation.
//
// Digging a bit deeper we can find more info:
//
// Select the `cortex_m_rt::Hardfault` frame in the CALL STACK
//
// Expand the `Local` you will see the content of `ef`
//
// r0: 0x2fffffff
// ...
// pc: 0x0800020a
// ...
//
// r0 is the address of the failing access (outside region)
// pc is address of the failing instruction
//
// Select the `rtic_crash::app::init` frame in the CALL STACK
// > disass
//    0x08000206 <+0>:	mvn.w	r0, #3489660928	; 0xd0000000
// => 0x0800020a <+4>:	ldr	r0, [r0, #0]
//    0x0800020c <+6>:	cpsie	i
//    0x0800020e <+8>:	bx	lr
//
// Indeed on address 0x0800020a there is a `ldr` instruction
// (Stemming from the `read_volatile` function).
//
// So we can actually track down the cause of error.
//
// Discussion:
//
// In this case it was our code that directly caused the Hard Fault.
//
// In other cases it might not be that easy.
// What about a DMA access to a faulty memory location?
// In that case it is the faulty configuration of the DMA that
// indirectly caused the error.
//
// You need to understand both the HW and the SW to write
// embedded applications, this makes it challenging.
