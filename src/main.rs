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

use collections::Vec;

mod lang_items;
mod vector_table;
mod exception;
mod led;
mod button;
mod event;
mod systick;
mod led_flash_controller;

extern crate critical_section_arm;
use critical_section_arm::CriticalSection;

use event::Event;
use led_flash_controller::LedFlashController;

fn delay(count: u32) {
    let mut total = 0;
    for i in 0..count {
        total += i;
    }
}

fn flash_green(count: u32) {
    for i in 0..count {
        led::set_green();
        delay(20000);
        led::set_off();
        delay(20000);
    }
}

fn flash_blue(count: u32) {
    for i in 0..count {
        led::set_blue();
        delay(20000);
        led::set_off();
        delay(20000);
    }
}

extern {
    fn zero_fill_bss();
    fn copy_initialized_data();
}

fn toggle_led(is_on: bool) -> bool {
    if is_on {
        led::set_off();
        false
    } else {
        led::set_blue();
        true
    }
}

struct StateMachine {
    flash_count: usize,
    flash_in_progress: bool,
    pause_time_remaining: usize,
}

impl StateMachine {
    fn new() -> StateMachine {
        StateMachine {
            flash_count: 1,
            flash_in_progress: false,
            pause_time_remaining: 0,
        }
    }
    
    fn execute(&mut self, event: &Event) -> Option<Event>{
        
        // All times are in 10 Hz ticks.
        const LED_ON_TIME: usize = 4;
        const LED_OFF_TIME: usize = 3;
        const WAIT_TIME: usize = 20;
        
        match *event {
            Event::ButtonPress => {
                self.flash_count += 1;
                None
            },
            Event::TimeTick if (!self.flash_in_progress && self.pause_time_remaining == 0) => {
                // Start the next flash.
                self.flash_in_progress = true;
                Some(Event::FlashLed { count: self.flash_count, on_time: LED_ON_TIME, off_time: LED_OFF_TIME })
            },
            Event::TimeTick if self.pause_time_remaining > 0 => {
                //We're waiting to start the next flash.
                self.pause_time_remaining -= 1;
                None
            },
            Event::FlashLedDone => {
                self.flash_in_progress = false;
                self.pause_time_remaining = WAIT_TIME;
                None
            }
            Event::LedTurnOn => {
                led::set_red();
                None
            },
            Event::LedTurnOff => {
                led::set_off();
                None
            },
            _ => None,
        }
    }
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
