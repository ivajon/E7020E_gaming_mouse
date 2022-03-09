# RTIC on the STM32F4xx Nucleo board

All tooling have been developed and tested under Linux. Any modern Linux distro should work, we usually recommend Arch linux as it provides a great package manager with rolling releases. If you want to run Arch, but don't want to install everything from scratch, you may opt for [Manjaro](https://manjaro.org/) or [Endeavour](https://endeavouros.com/). You will get the best user experience by a native install, but you may run Linux under a VM like virtualbox, or vmware (the player is free). You should install the guest extensions, to get better graphics performance (and perhaps better USB forwarding). Since you will connect your Nucleo using USB, you must make sure that USB port forwarding works (the Nucleo stlink programmer is a USB2 device running in full speed 12MBit).

This repo will be updated with more information throughout the course so please check the `CHANGELOG.md` and recent commits to see what has changed. (You should `pull` the upstream to keep your repository updated.) If you have suggestions to further improvements, please raise an issue and/or create a merge/pull request.

## Rust

We assume Rust to be installed using [rustup](https://www.rust-lang.org/tools/install).

Additionally you need to install the `thumbv7em-none-eabihf` target.

```shell
> rustup target add thumbv7em-none-eabihf 
```

You also need [cargo-binutils](https://github.com/rust-embedded/cargo-binutils), for inspecting the generated binaries. You install Rust applications through `cargo`

```shell
> cargo install cargo-binutils
```

There are a number of other useful [cargo subcommands](https://github.com/rust-lang/cargo/wiki/Third-party-cargo-subcommands).

- [cargo-bloat](https://crates.io/crates/cargo-bloat), for info on the size of different sections of the generated binary,
- [cargo-tree](https://crates.io/crates/cargo-tree) (that list your dependency tree), etc.

## For RTT tracing

Install the [probe-run](https://crates.io/crates/probe-run) tool.

```shell
> cargo install probe-run
```

Install the [cargo-embed](https://github.com/probe-rs/cargo-embed) tool.

```shell
> cargo install cargo-embed
```

Or for the latest git version, with improved support for blocking writes in case buffer space is depleted.

```shell
> cargo install --git https://github.com/probe-rs/cargo-embed.git
```

## For `itm` tracing in terminal

The SWO output pin of the MCU is used to transmit trace information over a single serial wire. The onboard Nucleo `stlink` programmer has an SWO input pin (by default connected to the target MCU). You can setup `openocd` to forward the serial communication to a file (or pipe). In order to view the trace as text, the serial stream needs to be decoded by some tool (e.g., the `itmdump` utility or the built in decoder in `Cortex Debug`). To install `itmdump`:

```shell
> cargo install itm
```

## For programming and low-level `gdb` based debugging

Use your Linux package manager to install the following tools:

- `stlink`, this package will install programming utilities like `st-flash` (useful if you need to recover a bricked target by erasing the flash), and setup `udev` rules, allowing you to access the USB device without `sudo`. Install may require you to login/logout to have new `udev` rules applied.

- `arm-none-eabi-gdb` (Arch), or `gdb-multiarch` (Ubuntu etc.). This tool allows you to program (flash) and debug your target.

- `openocd`, this tool allows the host to connect to the (stlink) programmer. The scripts used assume the latest `openocd-git` (0.11.0+dev).

  Under Arch: The official package (as of 2022-02-13) relates to a released 0.11.0 version of `openocd`, and will not likely work, you need a `dev` version. You can either install from source [openocd](https://github.com/openocd-org/openocd), or easier using a prepared package build:
  - Clone my repo [openocd-git](https://github.com/perlindgren/openocd-git/). (The official `aur` is outdated, so I made a fix.)

  - Install the package:

    ```shell
    > makepkg -si
    ```

  - Trouble shooting. If it fails to compile you might be missing some dependency provided by the `base-devel` package.
  
    ```shell
    > pacman -S base-devel
    ```

  Other Linux distributions: Either install from source or find a package relating to the git version of `openocd`.

- [stlink](https://www.st.com/en/development-tools/stsw-link007.html), firmware for the `st-link` programmer on your Nucleo board. You can obtain it free from ST web site, or under Arch use the `aur` package [stsw-link007](https://aur.archlinux.org/packages/stsw-link007). To run the firmware update in Arch:
  
  ```shell
  >STLinkUpgrade
  ```

  Once the Java application started you can flash a new firmware.
  Choose `Open in update mode`, leave `Change type` unchecked and click `Upgrade`.

## Editor

You may use any editor of choice. We recommend `vscode`, the `rust-analyzer` and `Cortex Debug` plugins. There are many other great plugins for improved git integration, `.md` linting and preview etc.

In the `.vscode` folder of this project, you find a number of configuration files (`launch.json` for target debugging, `tasks.json` for building, etc.), that facilitates embedded Rust development.

## Useful Resources

- Nucleo 64
  - [UM1724 - stm32 Nucleo-64](https://www.st.com/resource/en/user_manual/dm00105823-stm32-nucleo64-boards-mb1136-stmicroelectronics.pdf).
  - [Nucleo 64 Schematics](https://www.st.com/resource/en/schematic_pack/nucleo_64pins_sch.zip) (The file MB1136.pdf is the schematics in pdf.)
  - [stm32f4xx_hal](https://docs.rs/stm32f4xx-hal/0.8.3/stm32f4xx_hal/) documentation of the HAL API, and [git repository](https://github.com/stm32-rs/stm32f4xx-hal).

- STM32F01/FO11
  - [RM0383 - F411 Reference Manual](https://www.st.com/resource/zh/reference_manual/dm00119316-stm32f411xce-advanced-armbased-32bit-mcus-stmicroelectronics.pdf) 
  - [RM0368 - F401 Reference Manual](https://www.st.com/resource/en/reference_manual/dm00096844-stm32f401xbc-and-stm32f401xde-advanced-armbased-32bit-mcus-stmicroelectronics.pdf)
  - [PM0214 - M4 Programming manual](https://www.google.com/url?sa=t&rct=j&q=&esrc=s&source=web&cd=&ved=2ahUKEwjOtd645OTtAhXEHXcKHdwYCoQQFjAAegQIBhAC&url=https%3A%2F%2Fwww.st.com%2Fresource%2Fen%2Fprogramming_manual%2Fdm00046982-stm32-cortex-m4-mcus-and-mpus-programming-manual-stmicroelectronics.pdf&usg=AOvVaw0n3XXybtMMDbifhDZse1Pl)

- PixArt PMW33xx Optical Navigation Chip
  - [PMW3389DM-T3QU](https://www.google.com/url?sa=t&rct=j&q=&esrc=s&source=web&cd=&ved=2ahUKEwicx5OA9eTtAhWC-yoKHVfKAJ0QFjAAegQIBhAC&url=https%3A%2F%2Fwww.pixart.com%2F_getfs.php%3Ftb%3Dproduct%26id%3D4%26fs%3Dck2_fs_cn&usg=AOvVaw1A1rR533Pt-7EgnVSS-_ch), optical navigation chip
  - [Jack Enterprise Breakout Board](https://www.tindie.com/products/jkicklighter/pmw3389-motion-sensor/), an example design with software linked.

- General Embedded
  - [Introduction to SPI](https://www.analog.com/en/analog-dialogue/articles/introduction-to-spi-interface.html#), a short introduction to the SPI interface.

---

## Examples

### Terminal trace using RTT

Select the runner in `.cargo/config.toml`:

```shell
runner = "probe-run --chip STM32F411RETx"
```

Run your application from terminal:

```shell
> cargo run --example rtt_rtic_hello
```

Tracing of panics is supported by the `panic_rtt` crate.

---

### VSCODE based debug and trace

Some simple bare metal examples for you to try out before starting to run your own code: (Here we assume the tools mentioned above have been successfully installed into their default locations. If you installed manually, you may need to tweak paths accordingly, ask in the `software` channel on Discord if you run into some trouble.)

Using `vscode` just press F5 to launch and debug the program in the currently active `vscode` window.

- `rtic_hello.rs`, this example uses semihosting to print the output terminal. In `vscode`:
  - `Terminal` pane `gdb-server` shows the `openocd` session (your semihosting trace will be displayed there).
  - `DEBUG CONSOLE` pane shows the `gdb` session. You can use this to interact directly with the target (over openocd).

- `itm_rtic_hello.rs`, this examples uses the ITM trace to print to trace channel 0. The `Terminal` pane `SWO:ITM[port:0] console]` will display the ITM trace.

- `rtic_panic.rs`, this example shows how to trace panic messages (in this case over semihosting).  Open the `Terminal` pane and select `gdb-server` (which is the `openocd` console).
  
---

### Terminal trace using semihosting

Select the runner in `.cargo/config.toml` according to your `gdb` install, e.g.:

```shell
runner = "arm-none-eabi-gdb -q -x openocd.gdb"
```

Start `openocd` in a separate terminal:

```shell
> openocd -f openocd.cfg
```

Run the application: e.g.

```shell
> cargo run rtic_hello
```

The `openocd.gdb` script is used to setup the behavior of the debug session, you may change it to your liking. (The default script puts breakpoints at `DefaultHandler`, `HardFault` and `main`, edit it for your needs.)

```shell
> cargo run rtic_hello
```

---

### Terminal trace using ITM

Setup the runner as for previous example.

To setup a fifo for the ITM stream and start sniffing:

```shell
> mkfifo /tmp/itm.fifo
> itmdump -F -f /tmp/itm.fifo
```

The `-F` instructs `itmdump` to follow the stream.

Start the `openocd` session in a separate terminal.

```shell
> openocd -f openocd.cfg
```

Start the `gdb` session in a separate terminal.

```shell
> cargo run ....
```

This, in this scenario you will have three terminals running:

- `itmdump`, to decoded and echo the ITM stream,
  
- `openocd`, to communicate to the programmer and provide a gdb-server,
  
- `arm-none-eabi-gdb`, to debug our application.

ITM traces should now appear in the `itmdump` terminal and semihosting traces in the `openocd` terminal.

---

### Gdb the GNU debugger

Gdb offers a lot of functionality, you may even write scripts in python to automate debugging. For a summary of commands see e.g., [gdb-refcard](http://fac-staff.seattleu.edu/elarson/web/Linux/gdb-refcard.pdf).

Many of the commands can be executed from the `DEBUG CONSOLE` in `vscode`.

---

### Concluding remarks on debug and trace

Chose the approach and tools dependent on your application needs. If plain tracing  is sufficient chose `probe run`. If you need to interactively debug your application, use `gdb` based trace and debug instead. ITM is preferred over semihosting for real-time applications.

You may also look at [defmt](https://crates.io/crates/defmt), it supports deferred formatting to reduce the run-time overhead (on the target).

---

### Exercises

Bare metal programming:

- `examples/rtic_bare1.rs`, in this exercise you learn about debugging, inspecting the generated assembly code, inline assembly, and about checked vs. unchecked (wrapping) arithmetics. Provides essential skills and understanding of low level (bare metal) programming.
- `examples/rtic_bare2.rs`, in this exercise you learn how to measure execution time using raw timer access.
- `examples/rtic_bare3.rs`, here you learn more about RTIC timing semantics and timing abstractions.
- `examples/rtic_bare4.rs`, in this exercise you will encounter a simple bare metal peripheral access API. The API is very unsafe and easy to misuse.
- `examples/rtic_bare5.rs`, here you will write your own C-like peripheral access API. This API is much safer as you get control over bit-fields in a well defined way, thus less error prone.

---

## Troubleshooting

---

### Fail to connect or program (flash) your target

- Make sure you have the latest version of the [stlink](https://www.st.com/en/development-tools/stsw-link007.html) firmware (V2J39M27 or later).

- Check that your stlink Nucleo programmer is found by the host.

  ```shell
  > lsusb
  ...
  Bus 003 Device 013: ID 0483:374b STMicroelectronics ST-LINK/V2.1
  ...
  ```

  If not check your USB cable. Notice, you need a USB data cable (not a USB charging cable).
  If the problem is still there, there might be a USB issue with the host (or VM if you run Linux under a VM that is).

- If you get a connection error similar to the below:

  ```shell
  > openocd -f openocd.cfg
  Open On-Chip Debugger 0.11.0+dev-00562-g5ab74bde0 (2022-02-09-11:34)
  Licensed under GNU GPL v2
  For bug reports, read
          http://openocd.org/doc/doxygen/bugs.html
  Info : auto-selecting first available session transport "hla_swd". To override use 'transport select <transport>'.
  Info : The selected transport took over low-level target control. The results might differ compared to plain JTAG/SWD
  Info : Listening on port 6666 for tcl connections
  Info : Listening on port 4444 for telnet connections
  Info : clock speed 2000 kHz
  Info : STLINK V2J39M27 (API v2) VID:PID 0483:374B
  Info : Target voltage: 3.234098
  Info : stm32f4x.cpu: Cortex-M4 r0p1 processor detected
  Info : stm32f4x.cpu: target has 6 breakpoints, 4 watchpoints
  Info : starting gdb server for stm32f4x.cpu on 3333
  Info : Listening on port 3333 for gdb connection
  Error: jtag status contains invalid mode value - communication failure
  Polling target stm32f4x.cpu failed, trying to reexamine
  Examination failed, GDB will be halted. Polling again in 100ms
  Info : Previous state query failed, trying to reconnect
  Error: jtag status contains invalid mode value - communication failure
  Polling target stm32f4x.cpu failed, trying to reexamine 
  ```

  - First thing to try is holding the reset button while connecting.

  - If this does not work you can try to erase the flash memory (the program running on the STM32F401/F11).

    ``` shell
    > st-flash erase
    st-flash 1.7.0
    2021-06-23T14:07:35 INFO common.c: F4xx (Dynamic Efficency): 96 KiB SRAM, 512 KiB flash in at least 16 KiB pages.
    Mass erasing.......
    ```

  - If this still does not work you can connect `Boot0` to `VDD` (found on CN7 pins 7, and 5 respectively). Unplug/replug the Nucleo and try to erase the flash as above.
  
  - If this still does not work, the Nucleo might actually been damaged, or that the problem is the usb-cable or host   machine related.
