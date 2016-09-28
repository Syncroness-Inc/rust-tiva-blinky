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

mod led;
mod button;
mod event;

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
    led::init();
    button::init();
    
    let mut flash_count = 1;
    
    loop {
        match event::get() {
            Some(e) => match e {
                event::Event::ButtonPress => flash_count += 1,
                _ => {},
            },
            None => {},
        }
        flash_green(flash_count);
        delay(60000);
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
    static EXCEPTIONS: [Option<fn() -> !>; 14] = [
        Some(::exception::nmi),  // NMI
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
        Some(::exception::default_handler) // Systick
    ];
    
    #[link_section = ".interrupts"]
    static INTERRUPT_HANDLERS: [Option<fn()>; 33] = [
        Some(::button::handler), // GPIO A
        Some(::button::handler), // GPIO B
        Some(::button::handler), // GPIO C
        Some(::button::handler), // GPIO D
        Some(::button::handler), // GPIO E
        None, // UART0 Rx and Tx
        None, // UART1 Rx and Tx
        None, // SSI0 Rx and Tx
        None, // I2C0 Master and Slave
        None, // PWM Fault
        None, // PWM Generator 0
        None, // PWM Generator 1
        None, // PWM Generator 2
        None, // Quadrature Encoder 0
        None, // ADC Sequence 0
        None, // ADC Sequence 1
        None, // ADC Sequence 2
        None, // ADC Sequence 3
        None, // Watchdog timer
        None, // Timer 0 subtimer A
        None, // Timer 0 subtimer B
        None, // Timer 1 subtimer A
        None, // Timer 1 subtimer B
        None, // Timer 2 subtimer A
        None, // Timer 2 subtimer B
        None, // Analog Comparator 0
        None, // Analog Comparator 1
        None, // Analog Comparator 2
        None, // System Control (PLL, OSC, BO)
        None, // FLASH Control
        Some(::button::handler), // GPIO Port F
        Some(::button::handler), // GPIO Port G
        Some(::button::handler), // GPIO Port H
    ];
}
