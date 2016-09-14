
This is an example project to blink an LED in Rust on the Tiva C Series Launchpad board (EK-TM4C123GXL).

It should be relatively easier to use this with other ARM Cortex-M processors.

## How to use with a different processor.
- Get a new target specification file for your processor type, like one from [here](https://japaric.github.io/copper/details/target.html).
- Update the linker script (**layout.ld**) to have the correct size and addresses of FLASH and RAM.

## To Do
- Zero out RAM during initialization.
- Add initialization code to copy static RAM variables, so we can have RAM values initialized to values.
- Add support for `for` loops. Now, if you try to use `for` loop, you get a linker problem with iterators needing `_Unwind_Resume` to be defined.