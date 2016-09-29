
const SYSCTL_PERIPH_GPIOF: u32 = 0xf0000805;
const GPIO_PORTF_BASE: u32 = 0x40025000;
const GPIO_PIN_0: u8 = 0x01;  // GPIO pin 0
const GPIO_PIN_1: u8 = 0x02;  // GPIO pin 1
const GPIO_PIN_2: u8 = 0x04;  // GPIO pin 2
const GPIO_PIN_3: u8 = 0x08;  // GPIO pin 3
const GPIO_PIN_4: u8 = 0x10;  // GPIO pin 4
const INT_GPIOF: u32 = 46;
const GPIO_FALLING_EDGE: u32 = 0;
const GPIO_STRENGTH_2MA: u32 = 0;
const GPIO_PIN_TYPE_STD_WPU: u32 = 0xA;
const GPIO_INT_PIN_0: u32 = 0x1;
const GPIO_INT_PIN_4: u32 = 0x10;

const BUTTON_PERIPHERAL: u32 =  SYSCTL_PERIPH_GPIOF;
const BUTTON_PORT: u32 = GPIO_PORTF_BASE;
const BUTTON_1_PIN: u8 = GPIO_PIN_4;
const BUTTON_2_PIN: u8 = GPIO_PIN_0;
const BUTTON_PORT_INTERRUPT: u32 = INT_GPIOF;
const BUTTON_1_INTERRUPT: u32 = GPIO_INT_PIN_4;
const BUTTON_2_INTERRUPT: u32 = GPIO_INT_PIN_0;

extern {
    fn SysCtlPeripheralEnable(ui32Peripheral: u32);
    fn SysCtlDelay(ui32Count: u32);
    
    fn GPIOPinTypeGPIOInput(ui32Port: u32, ui8Pins: u8);
    fn GPIOPadConfigSet(ui32Port: u32, ui8Pins: u8, ui32Strength: u32, ui32PadType: u32);
    
    fn GPIOIntTypeSet(ui32Port: u32, ui8Pins: u8, ui32IntType: u32);
    fn GPIOIntEnable(ui32Port: u32, ui8Pins: u8);
    fn GPIOIntClear(ui32Port: u32, ui32IntFlags: u32);
    fn IntEnable(ui32Interrupt: u32);
    fn IntDisable(ui32Interrupt: u32);
}

use super::event;

pub fn init () {
    unsafe {
        
        //Need to unlock PF0.
        
        SysCtlPeripheralEnable(BUTTON_PERIPHERAL);
        GPIOPinTypeGPIOInput(BUTTON_PORT, BUTTON_1_PIN | BUTTON_2_PIN);
        GPIOPadConfigSet(BUTTON_PORT, BUTTON_1_PIN | BUTTON_2_PIN,
                         GPIO_STRENGTH_2MA, GPIO_PIN_TYPE_STD_WPU);
        
    	GPIOIntTypeSet(BUTTON_PORT, BUTTON_1_PIN, GPIO_FALLING_EDGE);
    	//GPIOIntTypeSet(BUTTON_PORT, BUTTON_2_PIN, GPIO_FALLING_EDGE);
    	GPIOIntEnable(GPIO_PORTF_BASE, BUTTON_1_PIN /*| BUTTON_2_PIN*/);
    	IntEnable(BUTTON_PORT_INTERRUPT);
    }
}

#[allow(dead_code)]
pub fn handler () {
    unsafe {
        // Clear the interrupt.
        GPIOIntClear(BUTTON_PORT, BUTTON_1_INTERRUPT /*| BUTTON_2_INTERRUPT*/);
        
        // Wait for the interrupt to clear.
        //SysCtlDelay(3);
        
        // Raise an event.
        event::raise(event::Event::ButtonPress);
    }
}