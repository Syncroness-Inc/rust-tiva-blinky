/*
    Configure the SysTick to generate a periodic interrupt. This is super hardware dependent
    because it depends on the clock configuration (including crystal).
*/

use super::event;

const SYSCTL_SYSDIV_1: u32 = 0x07800000;
const SYSCTL_USE_OSC: u32 = 0x00003800;
const SYSCTL_OSC_MAIN: u32 = 0x00000000;
const SYSCTL_XTAL_16MHZ: u32 = 0x00000540;

extern {
    fn SysCtlClockSet(config: u32);
    fn SysCtlClockGet() -> u32;
    fn SysTickPeriodSet(period: u32);
    fn SysTickIntEnable();
    fn SysTickEnable();
}

pub fn init (frequency_hz: u32) {
    unsafe {
        // Configure the clock.
        SysCtlClockSet(SYSCTL_SYSDIV_1 | SYSCTL_USE_OSC | SYSCTL_OSC_MAIN | SYSCTL_XTAL_16MHZ);
        
        // Set the SysTick to generate an interrupt at the configured rate.
        SysTickPeriodSet(SysCtlClockGet()/frequency_hz);

        // Enable the system tick interrupt.
        SysTickIntEnable();
        
        // Enable the system tick.
        SysTickEnable();
    }
}

#[allow(dead_code)]
pub fn handler () {
    event::raise(event::Event::TimeTick);
}