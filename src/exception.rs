/*
    Exception handlers. For now, these just set a breakepoint.
*/

#[allow(dead_code)]
    
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