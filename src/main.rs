#![feature(lang_items)]

// We won't use the usual `main` function. We are going to use a different "entry point".
#![no_main]

// We won't use the standard library because it requires OS abstractions like threads and files and
// those are not available in this platform.
#![no_std]

// For using in-line assembly.
#![feature(asm)]

mod led;

fn delay() {
    let mut total = 0;
    for count in 0..40000 {
        total += count;
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

    led::init();
    
    loop {
        led::set_green();
        delay();
        led::set_off();
        delay();
    }
}

// Finally, we need to define some "lang items" we are _not_ going to use, but that `rustc` demands
// anyway. As we are not going to use the functionality they provide (panic/unwinding) we can left
// their definitions empty.
mod lang_items {
    #[lang = "panic_fmt"]
    extern fn panic_fmt() {}

    #[lang = "eh_personality"]
    fn eh_personality() {}
}

// Set a breakpoint if we get an exception.
#[allow(dead_code)]
mod exception {
    pub fn handler() -> ! {
        unsafe {
            asm!("bkpt");
        }

        loop {}
    }
}

// We need to put the reset and exception vectors at the right spot in memory, but they're never
// called directly, so the compiler thinks this is dead code.
#[allow(dead_code)]
mod vector_table {
    #[link_section = ".reset"]
    static RESET: fn() -> ! = ::start;
    
    #[link_section = ".exceptions"]
    static EXCEPTIONS: [Option<fn() -> !>; 14] = [Some(::exception::handler),  // NMI
                                                  Some(::exception::handler),  // Hard fault
                                                  Some(::exception::handler),  // Memory management fault
                                                  Some(::exception::handler),  // Bus fault
                                                  Some(::exception::handler),  // Usage fault
                                                  None, // Reserved
                                                  None, // Reserved
                                                  None, // Reserved
                                                  None, // Reserved
                                                  Some(::exception::handler),  // SVCall
                                                  None, // Reserved for Debug
                                                  None, // Reserved
                                                  Some(::exception::handler),  // PendSV
                                                  Some(::exception::handler)]; // Systick
}