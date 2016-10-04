
This is an example project to blink an LED in Rust on the Tiva C Series Launchpad board (EK-TM4C123GXL).

It should be relatively easier to use this with other ARM Cortex-M processors.

## Vagrant

Of course there is a Vagrantfile that sets up the environment for you. Start it up with `vagrant up`. Log in with `vagrant ssh`.

## Building

There is a rakefile to make building easier. Build and load the example on the board with `rake load`. This will build the TivaWare library too.

## Debugging with GDB

Log in to the vagrant instance from two separate terminals. From one run `rake ocd`. This starts openocd for communicating with the board. From the other terminal run `rake gdb`. This will start GDB, connect to the board and load the application.

## How to use with a different processor.
- Get a new target specification file for your processor type, like one from [here](https://japaric.github.io/copper/details/target.html).
- Update the linker script (**layout.ld**) to have the correct size and addresses of FLASH and RAM.

## Unit tests

Run the unit test with `rake test`. This runs the all of the test on the host (not the target board).

## To Do
- Move RAM initialization (zero_fill_bss and copy_initialized_data) into assembly, or Rust.
- The heap_start and heap_end addresses are hardcoded in the syscalls.c file. A much better strategy would be have them exported from the linker script, and have the linker do some size checking against the data, bss and whatever else are in RAM.
- Figure out how to build in release mode. This requires working around warnings for dead code.
- Create separate TivaWare library.
- Move vector table exception handling to it's own module.