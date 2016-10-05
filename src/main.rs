#![feature(lang_items)]

// Conditional enable/disable a bunch of options here. This is used to configure the application
// differently for 1) running on the target board and 2) running tests on the host. In general,
// we're using cfg_attr to only set these attributes if target_os is "none." This is the case
// where we're building for the target.

// We won't use the usual `main` function. We are going to use a different "entry point".
#![cfg_attr(target_os = "none", no_main)]

// We won't use the standard library because it requires OS abstractions like threads and files and
// those are not available in this platform.
#![cfg_attr(target_os = "none", no_std)]

// Allow use of these libraries individually (this is unstable at this point).
#![cfg_attr(target_os = "none", feature(alloc))]
#![feature(collections)]

// For using in-line assembly.
#![feature(asm)]
#![cfg_attr(target_os = "none", feature(asm))]

// Allow using types which implement Drop to be used as globals.
#![feature(drop_types_in_const)]

// Pull in our custom allocator.
#[cfg(target_os = "none")]
extern crate libc_allocator;

// We need this for the dynamic allocation on the heap used by vector.
#[cfg(target_os = "none")]
extern crate alloc;

// Even though we are not using the standard library, we can include individual things like vectors.
#[macro_use(vec)]
extern crate collections;

// For creating critical sections where we disable interrupts.
extern crate critical_section_arm;

mod lang_items;
mod vector_table;
mod exception;
mod led;
mod button;
mod event;
mod systick;
mod state_machine;
mod led_flash_controller;

use led_flash_controller::LedFlashController;
use state_machine::StateMachine;

extern {
    fn zero_fill_bss();
    fn copy_initialized_data();
}

// Conceptually, this is our program "entry point". It's the first thing the microcontroller will
// execute when it (re)boots. (As far as the linker is concerned the entry point must be named
// `start` (by default; it can have a different name). That's why this function is `pub`lic, named
// `start` and is marked as `#[no_mangle]`.)
//
// Returning from this function is undefined because there is nothing to return to! To statically
// forbid returning from this function, we mark it as divergent, hence the `fn() -> !` signature.
#[no_mangle]
pub fn start() -> ! {

    unsafe {
        copy_initialized_data();
        zero_fill_bss();
    }

    event::init();
    systick::init(10); //Generate a time tick at 10 Hz.
    led::init();
    button::init();
    
    let mut state_machine = StateMachine::new();
    let mut led_flash_controller = LedFlashController::new();
    
    loop {
        match event::get() {
            Some(e) => {
                match state_machine.execute(&e) {
                    // If handling this event generates a new event, raise it to the system.
                    Some(next_event) => event::raise(next_event),
                    _ => (),
                }
                match led_flash_controller.process_event(&e) {
                    // If handling this event generates a new event, raise it to the system.
                    Some(next_event) => event::raise(next_event),
                    _ => (),
                }
            },
            None => {},
        }
    }
}

#[cfg(test)]
mod test {
    #[test]
    fn it_works() {
    }
}
