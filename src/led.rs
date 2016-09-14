const SYSCTL_PERIPH_GPIOF: u32 = 0xf0000805;
const GPIO_PORTF_BASE: u32 = 0x40025000;
const GPIO_PIN_1: u8 = 0x02;  // GPIO pin 1
const GPIO_PIN_2: u8 = 0x04;  // GPIO pin 2
const GPIO_PIN_3: u8 = 0x08;  // GPIO pin 3

const LED_PORT: u32 = GPIO_PORTF_BASE;
const RED_PIN: u8 = GPIO_PIN_1;
const BLUE_PIN: u8 = GPIO_PIN_2;
const GREEN_PIN: u8 = GPIO_PIN_3;

extern {
    fn SysCtlPeripheralEnable(ui32Peripheral: u32);
    fn GPIOPinTypeGPIOOutput(ui32Port: u32, ui8Pins: u8);
    fn GPIOPinWrite(ui32Port: u32, ui8Pins: u8, ui8Val: u8);
}

pub fn init () {
    unsafe {
        SysCtlPeripheralEnable(SYSCTL_PERIPH_GPIOF);
        GPIOPinTypeGPIOOutput(GPIO_PORTF_BASE, GPIO_PIN_1 | GPIO_PIN_2 | GPIO_PIN_3);
        GPIOPinWrite(GPIO_PORTF_BASE, GPIO_PIN_1 | GPIO_PIN_2 | GPIO_PIN_3, 0); //Turn off all the LEDs.
    }
}

pub fn set_blue () {
    unsafe {
        //Turn on the blue LED.
        GPIOPinWrite(LED_PORT, BLUE_PIN, BLUE_PIN);
         
        //Turn off the other LEDs.
        GPIOPinWrite(LED_PORT, RED_PIN | GREEN_PIN, 0);
    }
}

pub fn set_green () {
    unsafe {
        //Turn on the green LED.
        GPIOPinWrite(LED_PORT, GREEN_PIN, GREEN_PIN);
        
        //Turn off the other LEDs.
        GPIOPinWrite(LED_PORT, RED_PIN | BLUE_PIN, 0);
    }
}

pub fn set_red () {
    unsafe {
        //Turn on the blue LED.
        GPIOPinWrite(LED_PORT, RED_PIN, RED_PIN);
        
        //Turn off the other LEDs.
        GPIOPinWrite(LED_PORT, BLUE_PIN | GREEN_PIN, 0);
    }
}

pub fn set_off()
{
    unsafe {
        GPIOPinWrite(LED_PORT, RED_PIN | BLUE_PIN | GREEN_PIN, 0);
    }
}