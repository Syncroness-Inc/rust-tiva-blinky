// Implements the main state machine for the system.

use event::Event;
use led;

pub struct StateMachine {
    flash_count: usize,
    flash_in_progress: bool,
    pause_time_remaining: usize,
}

impl StateMachine {
    pub fn new() -> StateMachine {
        StateMachine {
            flash_count: 1,
            flash_in_progress: false,
            pause_time_remaining: 0,
        }
    }
    
    pub fn execute(&mut self, event: &Event) -> Option<Event>{
        
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