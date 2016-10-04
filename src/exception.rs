/*
    Exception handlers. For now, these just set a breakepoint.
*/

#[cfg(target_arch = "arm")]
fn breakpoint() {
    unsafe {
        asm!("bkpt");
    }
}

#[cfg(not(target_arch = "arm"))]
fn breakpoint() {
    unimplemented!();
}

#[allow(dead_code)]
pub fn default_handler() -> ! {
    breakpoint();
    loop {}
}

pub fn nmi() -> ! {
    breakpoint();
    loop {}
}

pub fn hard_fault() -> ! {
    breakpoint();
    loop {}
}

pub fn memory_fault() -> ! {
    breakpoint();
    loop {}
}

pub fn bus_fault() -> ! {
    breakpoint();
    loop {}
}

pub fn usage_fault() -> ! {
    breakpoint();
    loop {}
}