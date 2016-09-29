// We need to put the reset and exception vectors at the right spot in memory, but they're never
// called directly, so the compiler thinks this is dead code.
#[allow(dead_code)]

#[link_section = ".reset"]
static RESET: fn() -> ! = ::start;

#[link_section = ".exceptions"]
static EXCEPTIONS: [Option<fn() -> !>; 12] = [
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
];

#[link_section = ".interrupts"]
static INTERRUPT_HANDLERS: [Option<fn()>; 35] = [
    None, // PendSV
    Some(::systick::handler), // Systick
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