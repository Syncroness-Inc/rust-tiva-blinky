
use critical_section_arm::CriticalSection;
use collections::Vec;

pub enum Event {
    ButtonPress,
    TimeTick,
}

// The static vector which holds the queue of events. Since we'll be allocating this in
// the init function below and never deallocating it, this technically leaks memory. But
// this application is the only thing that runs on the target so it okay. There's no
// context beyond the context of the application.
static mut event_queue: Option<Vec<Event>> = None;

// Initialize the event queue. You must call this before any interrupts raise events.
pub fn init() {
    unsafe {
        if event_queue.is_none() {
            event_queue = Some(Vec::new());
        }
    }
}

pub fn raise(event: Event) {

    // Add the event to our queue. Do this in a critical section so that we
    // can't be interrupted. This means we can call this function outside of
    // an interrupt and the vector won't get corrupted. Also, because the ARM
    // Cortex allows for nested interrupts, disabling interrupts from within
    // the interrupt context prevents a nested interrupt from corrupting the 
    // vector.
    unsafe {
        let _cs = CriticalSection::new();
        get_queue().push(event);
    }
}

pub fn get() -> Option<Event> {
    // Get the most recent event.
    unsafe {
        let _cs = CriticalSection::new();
        get_queue().pop()
    }
}

// Extract a mutable reference to the event queue vector from the option type.
unsafe fn get_queue() -> &'static mut Vec<Event> {
    match event_queue {
        Some(ref mut q) => q,
        None => panic!(),
    }
}