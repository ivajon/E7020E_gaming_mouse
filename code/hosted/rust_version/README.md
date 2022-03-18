# HID-HOST

Experiment to read HID data.

## Arch Linux

Seems to be no additional required libraries to install.

## Udev rules (linux)

``` shell
> cp 99-hid.rules /etc/udev/rules.d
> sudo udevadm control --reload
> sudo udevadm trigger
```

Shamelessly borrowed from https://github.com/signal11/hidapi/blob/master/udev/99-hid.rules.

## Experiment 1

- Apply the udev rules to allow HID access from user space.
- Start the target (Iris) application (cargo run --example rtt-hid)
  - currently it enumerates values and sends them over HID to the host
- Start the host application (cargo run), it will
  - list available HID devices
  - open the Iris device (based on VID:HID) and
    - receive a buffer of 4 bytes
    - parse as a u32
    - print buffer and u32 value

## Notice

Since this crate is located inside of the `e7020e_rtic` crate, it will look at the top level `cargo.config` where we declare the `[build]` target default (to some embedded target).

However, this is a host side application (not running on the arm target), so we override this by explicitly setting the `[build]` target, in the local `.cargo/config.toml`. In this case it is a linux target. This needs to be changed if running under OSX or Windows. 

```shell
[build]
target = "x86_64-unknown-linux-gnu"    
```

## Disclaimer

The code has only been tested to work under linux. It should however be possible to compile/run under OSX/Windows as well, but you will need to setup `hidapi` for the platform at hand.
