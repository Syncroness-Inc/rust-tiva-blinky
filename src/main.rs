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

// Pull in our custom allocator.
extern crate libc_allocator;

// We need this for the dynamic allocation on the heap used by vector.
extern crate alloc;

// Even though we are not using the standard library, we can include individual things like vectors.
#[macro_use(vec)]
extern crate collections;
use collections::Vec;

mod led;

fn delay(count: u32) {
    let mut total = 0;
    for i in 0..count {
        total += i;
    }
}

fn flash_green(count: u32) {
    for i in 0..count {
        led::set_green();
        delay(10000);
        led::set_off();
        delay(10000);
    }
}

fn flash_blue(count: u32) {
    for i in 0..count {
        led::set_blue();
        delay(10000);
        led::set_off();
        delay(10000);
    }
}

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

    led::init();
    
    loop {
        let v = vec![1, 2, 3];
        let u = vec![4, 5, 6];
        for count in v {
            flash_green(count);
            delay(40000);
        }
        for count in u {
            flash_blue(count);
            delay(40000);
        }
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
    
    pub fn default_handler() -> ! {
        unsafe {
            asm!("bkpt");
        }
        loop {}
    }
    
    pub fn nmi() -> ! {
        unsafe {
            asm!("bkpt");
        }
        loop {}
    }
    
    pub fn hard_fault() -> ! {
        unsafe {
            asm!("bkpt");
        }
        loop {}
    }
    
    pub fn memory_fault() -> ! {
        unsafe {
            asm!("bkpt");
        }
        loop {}
    }
    
    pub fn bus_fault() -> ! {
        unsafe {
            asm!("bkpt");
        }
        loop {}
    }
    
    pub fn usage_fault() -> ! {
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
    static EXCEPTIONS: [Option<fn() -> !>; 14] = [Some(::exception::nmi),  // NMI
                                                  Some(::exception::hard_fault),  // Hard fault
                                                  Some(::exception::memory_fault),  // Memory management fault
                                                  Some(::exception::bus_fault),  // Bus fault
                                                  Some(::exception::usage_fault),  // Usage fault
                                                  None, // Reserved
                                                  None, // Reserved
                                                  None, // Reserved
                                                  None, // Reserved
                                                  Some(::exception::default_handler),  // SVCall
                                                  None, // Reserved for Debug
                                                  None, // Reserved
                                                  Some(::exception::default_handler),  // PendSV
                                                  Some(::exception::default_handler)]; // Systick
}

// Listed below are the five allocation functions currently required by custom
// allocators. Their signatures and symbol names are not currently typechecked
// by the compiler, but this is a future extension and are required to match
// what is found below.
//
// Note that the standard `malloc` and `realloc` functions do not provide a way
// to communicate alignment so this implementation would need to be improved
// with respect to alignment in that aspect.
