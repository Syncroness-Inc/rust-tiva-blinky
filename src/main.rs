#![feature(lang_items)]

// We won't use the usual `main` function. We are going to use a different "entry point".
#![no_main]

// We won't use the standard library because it requires OS abstractions like threads and files and
// those are not available in this platform.
#![no_std]

// Allow use of these libraries individually (this is unstable at this point).
#![feature(alloc)]
#![feature(collections)]

// For using in-line assembly.
#![feature(asm)]

// Allow using types which implement Drop to be used as globals.
#![feature(drop_types_in_const)]

// Pull in our custom allocator.
extern crate libc_allocator;

// We need this for the dynamic allocation on the heap used by vector.
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

extern crate critical_section_arm;
use critical_section_arm::CriticalSection;

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
    flash_count: u32,
    flashed_count: u32,
    led_flashing: bool,
    led_on: bool,
    timer: u32,
}

impl StateMachine {
    fn new() -> StateMachine {
        StateMachine {
            flash_count: 1,
            flashed_count: 0,
            led_flashing: false,
            led_on: false,
            timer: 0
        }
    }
    
    fn execute(&mut self, event: event::Event) {
        
        const LED_ON_TIME: u32 = 5;
        const LED_OFF_TIME: u32 = 2;
        
        match event {
            event::Event::ButtonPress => {
                self.flash_count += 1;
            },
            event::Event::TimeTick => {
                if !self.led_flashing {
                    // Wait 2 seconds.
                    self.timer += 1;
                    if self.timer >= 20 {
                        self.timer = 0;
                        self.led_flashing = true;
                        self.flashed_count = 0;
                    }
                } else {
                    // Turn on for one second, off for 1/2 second.
                    self.timer += 1;
                    if self.timer < LED_ON_TIME {
                        if !self.led_on {
                            // Initially turn on the LED.
                            self.led_on = toggle_led(self.led_on);
                        }
                    }
                    else if self.timer < (LED_ON_TIME + LED_OFF_TIME) {
                        if self.led_on {
                            // Turn off the LED.
                            self.led_on = toggle_led(self.led_on);
                        }
                    }
                    else {
                        // We're done with this flash.
                        self.flashed_count += 1;
                        self.timer = 0;
                    }
                    
                    if self.flashed_count >= self.flash_count {
                        self.led_flashing = false;
                    }
                }
            },
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
    
    loop {
        match event::get() {
            Some(e) => state_machine.execute(e),
            None => {},
        }
    }
}
