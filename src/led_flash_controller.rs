use ::event::Event;

#[derive(Debug)]
enum State {Inactive, Off, On}

impl Default for State {
    fn default() -> Self {
        State::Inactive
    }
}

#[derive(Default, Debug)]
pub struct LedFlashController {
    state: State,
    on_time: usize,
    off_time: usize,
    time_remaining: usize,
    flashes_remaining: usize,
}

impl LedFlashController {

    pub fn new() -> Self {
        LedFlashController { ..Default::default() }
    }

    fn handle_time_tick(&mut self) -> Option<Event> {
        
        //println!("{:?}", self);
        
        // Decrement the timer.
        if self.time_remaining > 0 {
            self.time_remaining -= 1;
        }
        
        match self.state {
            State::On if (self.time_remaining == self.on_time && self.flashes_remaining > 0) => {                
                Some(Event::LedTurnOn)
            },
            State::On if self.time_remaining == 0 => {
                self.flashes_remaining -= 1;
                self.state = State::Off;
                self.time_remaining = self.off_time;
                Some(Event::LedTurnOff)
            },
            State::Off if (self.time_remaining == 0 && self.flashes_remaining > 0) => {
                self.time_remaining = self.on_time;
                self.state = State::On;
                Some(Event::LedTurnOn)
            },
            State::Off if (self.time_remaining == 0 && self.flashes_remaining == 0) => {
                self.state = State::Inactive;
                Some(Event::FlashLedDone)
            },
            _ => None,
        }
    }
    
    fn handle_led_flash_request (&mut self, count: usize, on_time: usize, off_time: usize) -> Option<Event> {
        // Request to flash the LED.
        self.on_time = on_time;
        self.off_time = off_time;
        self.state = State::On;
        self.time_remaining = on_time + 1;
        self.flashes_remaining = count;
        
        // There is no additional event raised.
        None
    }

    pub fn process_event(&mut self, event: &Event) -> Option<Event> {
        match *event {
            Event::FlashLed{ count, on_time, off_time } => { self.handle_led_flash_request(count, on_time, off_time) },
            Event::TimeTick => { self.handle_time_tick() },
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use event::Event;
    
    fn tick_time (controller: &mut LedFlashController, count: usize) -> Option<Event>{
        let mut last_event: Option<Event> = None;
        for _ in 0 .. count {
            last_event = controller.process_event(&Event::TimeTick)
        }
        last_event
    }
    
    #[test]
    fn given_an_led_flash_has_been_requested_with_zero_flashes_when_the_next_tick_occurs_then_the_led_is_turned_on() {
        // given
        let mut c = LedFlashController { ..Default::default() };
        let event = c.process_event(&Event::FlashLed{ count: 0, on_time: 0, off_time: 0 });
        assert_eq!(None, event);
    }
    
    #[test]
    fn given_an_led_flash_has_been_requested_when_the_next_tick_occurs_the_led_is_turned_on() {
        // given
        let mut c = LedFlashController { ..Default::default() };
        c.process_event(&Event::FlashLed{ count: 1, on_time: 1, off_time: 1 });
        let event = c.process_event(&Event::TimeTick);
        assert_eq!(Event::LedTurnOn, event.unwrap());
    }
    
    #[test]
    fn given_an_led_flash_has_started_and_the_on_time_isnt_over_when_the_next_tick_occurs_then_the_led_is_not_turned_on_again() {
        // given
        let mut c = LedFlashController { ..Default::default() };
        c.process_event(&Event::FlashLed{ count: 1, on_time: 2, off_time: 1 });
        c.process_event(&Event::TimeTick);               //Turn on with tick 0.
        let event = c.process_event(&Event::TimeTick);   //Tick 1.
        assert_eq!(None, event);
    }
    
    #[test]
    fn given_an_led_flash_has_started_when_the_last_tick_occurs_then_the_led_is_turned_off() {
        // given
        let mut c = LedFlashController { ..Default::default() };
        c.process_event(&Event::FlashLed{ count: 1, on_time: 2, off_time: 1 });
        c.process_event(&Event::TimeTick);               //Turn on.
        let event = tick_time(&mut c, 2);               //Generate 2 ticks.
        assert_eq!(Event::LedTurnOff, event.unwrap());
    }
    
    #[test]
    fn it_turns_the_led_back_on_after_the_off_time_has_elapsed() {
        let mut c = LedFlashController { ..Default::default() };
        c.process_event(&Event::FlashLed{ count: 2, on_time: 4, off_time: 3}); // The flash count is 2.
        tick_time(&mut c, 1); // Tick the timer to start the display.
        tick_time(&mut c, 4); // This completes the on_time.
        
        // This completes the off time.
        let event = tick_time(&mut c, 3); 
        
        // The led should be turned back on after the off time has completed.
        assert_eq!(Event::LedTurnOn, event.unwrap());
    }
    
    #[test]
    fn it_only_turns_the_led_on_once_if_the_requested_count_is_one() {
        let mut c = LedFlashController { ..Default::default() };
        c.process_event(&Event::FlashLed{ count: 1, on_time: 4, off_time: 3}); // The flash count is 1.
        tick_time(&mut c, 1); // Tick the timer to start the display.
        tick_time(&mut c, 4); // This completes the on_time.
        
        // This completes the off time.
        let event = tick_time(&mut c, 3); 
        
        // The led should not be turned back on after the off time has completed.
        assert!(event != Some(Event::LedTurnOn));
    }
    
    #[test]
    fn it_flashes_the_led_the_correct_number_of_times() {
        let mut c = LedFlashController { ..Default::default() };
        c.process_event(&Event::FlashLed{ count: 5, on_time: 4, off_time: 3});
        
        let mut number_of_time_led_is_turned_on = 0;
        
        // Count up the number of times we expect to tick. This is once to start,
        // plus count * (on_time + off_time).
        let time_to_run = 1 + 5*(4+3);
        
        // Run this many time ticks, counting each LED turn on event.
        for _ in 0 .. time_to_run {
            match tick_time(&mut c, 1) {
                Some(Event::LedTurnOn) => number_of_time_led_is_turned_on += 1,
                _ => (),
            }
        }
        
        assert_eq!(5, number_of_time_led_is_turned_on);
    }
    
    #[test]
    fn it_never_turns_on_the_led_if_the_flash_count_is_zero() {
        let mut c = LedFlashController { ..Default::default() };
        c.process_event(&Event::FlashLed{ count: 0, on_time: 4, off_time: 3}); // The flash count is 0.
        
        // This normally turns on the LED here, but shouldn't now.
        let event = tick_time(&mut c, 1);
        
        assert_eq!(None, event);
    }
    
    #[test]
    fn it_raises_the_done_event_when_the_flashing_is_compelte() {
        let mut c = LedFlashController { ..Default::default() };
        c.process_event(&Event::FlashLed{ count: 1, on_time: 4, off_time: 3}); // The flash count is 1.
        tick_time(&mut c, 1); // Tick the timer to start the display.
        tick_time(&mut c, 4); // This completes the on_time.
        
        // This completes the off time.
        let event = tick_time(&mut c, 3); 
        
        assert_eq!(Event::FlashLedDone, event.unwrap());
    }
    
    #[test]
    fn it_only_raises_the_done_event_once_when_the_flashing_is_compelte() {
        let mut c = LedFlashController { ..Default::default() };
        c.process_event(&Event::FlashLed{ count: 1, on_time: 4, off_time: 3}); // The flash count is 1.
        tick_time(&mut c, 1); // Tick the timer to start the display.
        tick_time(&mut c, 4); // This completes the on_time.
        tick_time(&mut c, 3); // This completes the off time.
        
        // Tick one more time. We shouldn't get another done event.
        let event = tick_time(&mut c, 1);
        
        assert!(event != Some(Event::LedTurnOff));
    }
}