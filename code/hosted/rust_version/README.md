# Host program

This program is used to configure the mouse.


## Udev rules (linux)

``` shell
> cp 99-hid.rules /etc/udev/rules.d
> sudo udevadm control --reload
> sudo udevadm trigger
```

Shamelessly borrowed from https://github.com/signal11/hidapi/blob/master/udev/99-hid.rules.

## How to run and build

Build and run by typing `cargo run --example hid_comms --release` in a terminal.
To get help with what commands are available type `help` in the program.

## Config files

A config file is basically a text file with the desired commands in it one command on each row and `//` is used for comments. A couple of examples are available se [hello_world.cfg](hello_world.cfg) and [usefull.cfg](usefull.cfg). To load a configuration from file type `load-file` followed by the file name without the trailing `.cfg`.

## Disclaimer

The code has only been tested to work under linux.
