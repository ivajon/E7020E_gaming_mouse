# Changelog

## 2022-03-03

- src/pmw3389, (an experimental pmw3389 driver)
- rtic_pmw3389, (a simple example using the pmw3389 driver)

## 2022-03-02

- rtic_bare9_no_block, (an example of lock free access to multiple RTT channels). Uses `cargo-embed`.
- Embed.toml, (an example toml for `cargo-embed`)
- README.md, (updated with `cargo-embed` info)
  
## 2022-02-23

- stm32f4xx-hal bumped to 0.12.0 (only small fix to `rtic_bare6` needed)

- Examples
  - rtic_bare9, (example of a "good" reactive design)
  - rtic_usb_serial, (example of a VCP implementation)
  - rtic_usb_mouse, (example of a HID mouse implementation)

## 2022-02-22

- Examples
  - rtic_bare7, (using the stm32f4xx-hal)
  - rtic_bare7b, (using the embedded-hal and writing your own library)
  - rtic_bare8, (example of a "bad" (polling) design)

## 2022-02-18

- Examples
  - rtic_bare6, (clocking, measurements and function based peripheral API)
  - itm_rtic_blinky_48MHz, (showcase real-time ITM logging over SWO)

## 2022-02-17

- Examples
  - rtic_blinky, (showcase the stm32f4 API)
  - rtic_blinky_48mhz, (showcase the stm32fxx-hal for clocking)

## 2022-02-14

- Examples:
  - bare2..5, migrated to RTIC 1.0

## 2022-02-13

Migration and cleanup.

- Examples:
  - itm_rtic_hello, itm_rtic_hello2 (showcase ITM based single and dual port tracing)
  - rtic_crash, (Hard Fault handling)
  - rtic_hello, (showcase semihosting based tracing)
  - rtic_panic, (showcase panic handling)
  - rtt_rtic_hello, (showcase RTT based tracing)

- Exercises:
  - rtic_bare1, (arithmetics and panic handling)
  - rtic_bare2..5, in flux

## 2022-01-11

start migration to latest dependencies

- rtt_rtic_hello.rs, updated (rtt based tracing)
- rtic_hello.rs, updated (gdb based debug and semihosting)

## 2022-01-11, move to vesuvio, initial commit

## 2021-03-19

- `examples/itm_rtic_hello_48MHz.rs`, example to trace ITM, when processor runs at 48MHz, useful to debug USB applications.
  
- `.vscode/launch.json`, added 48MHz itm tracing profiles. (Now consistenly using `stlink.cfg`.)
  
## 2021-03-18

- `examples/usb-mouse.rs`, a very small example using external hid library.
  
## 2021-03-07

- `examples/rtic_bare7.rs`, using embedded HAL.
- `examples/rtic_bare8.rs`, serial communication, bad design.
- `examples/rtic_bare9.rs`, serial communication, good design.
  
## 2021-03-05

- `examples/rtic_bare6.rs`, setup and validate the clock tree.

## 2021-02-28

- `examples/rtic_bare2.rs`, raw timer access.
- `examples/rtic_bare3.rs`, timing abstractions.
- `examples/rtic_bare4.rs`, a simple bare metal peripheral access API.
- `examples/rtic_bare5.rs`, write your own C-like peripheral access API.

## 2021-02-26

- `examples/bare1.rs`, bare metal 101!
  
## 2021-02-23

- `examples/rtic_blinky.rs`, added instructions to terminal based debugging
  
## 2021-02-22

- `memory.x`, reduced flash size to 128k to match light-weight target
- `Cargo.toml`, updated dependencies to latest `stm32f4xx-hal/pac`

Some experiments (wip):

- `examples/rtt_rtic_i2c.rs`, spi emulation over i2c
- `src/pwm3389e`, driver using emulated spi

## 2021-02-16

- `rtt_rtic_usb_mouse` updated
  Notice, requires release build

## 2021-02-15

- Initial release for the e7020e course 2021
  