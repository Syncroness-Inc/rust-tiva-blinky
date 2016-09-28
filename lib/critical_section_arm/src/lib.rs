/*
    Control for enabling and disabling critical sections.
    
    Disable calls are refernce counted to allow for nesting. Uses RAII like is done in Zinc:
    https://github.com/hackndev/zinc/blob/master/src/hal/cortex_common/irq.rs
*/

#![no_std]
#![feature(asm)]

use core::ops::Drop;

pub struct CriticalSection {
    #[allow(dead_code)]
    contents: ()
}

impl CriticalSection {
    pub fn new() -> CriticalSection {
        unsafe { disable_interrupts(); }
        CriticalSection { contents: () }
    }
}

// When the critical section goes out of scope, re-enable interrupts.
impl Drop for CriticalSection {
    fn drop(&mut self) {
        unsafe { enable_interrupts(); }
    }
}

static mut critical_section_nest_level: usize = 0;

unsafe fn disable_interrupts() {
    asm!("cpsid i" :::: "volatile");
    critical_section_nest_level += 1;
}

unsafe fn enable_interrupts() {
    if critical_section_nest_level == 0 {
        // This is an error. Attempting to enable interrupts without them being disabled.
        panic!();
    }
    
    critical_section_nest_level -= 1;
    if critical_section_nest_level == 0 {
        // We've gotten back to the first place interrupts were disabled. Re-enable them now.
        asm!("cpsie i" :::: "volatile");
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
    }
}
