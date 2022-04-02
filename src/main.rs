#![no_std]  // Embedded mode (only lib-core)
#![no_main]

use core::panic::PanicInfo;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

#[arduino_hal::entry]
fn main() -> ! {
    // Access board peripherals
    let periph = arduino_hal::Peripherals::take().unwrap();
    // Access individual pins
    let pins = arduino_hal::pins!(periph);

    // Access d13 port
    let mut led = pins.d13.into_output();

    loop {
        led.toggle();
        arduino_hal::delay_ms(1000);
    }
}
