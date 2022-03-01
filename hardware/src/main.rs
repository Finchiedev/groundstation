#![no_std]
#![cfg_attr(not(test), no_main)]

use arduino_hal::prelude::*;
use core::panic::PanicInfo;
use embedded_hal::serial::Read;

// Explore using Knurling?
// Implement serial based off https://gist.github.com/bhansconnect/35f0a93c4c6c009728a8419d2701d615

#[arduino_hal::entry]
fn main() -> ! {
    let dp = arduino_hal::Peripherals::take().unwrap();
    let pins = arduino_hal::pins!(dp);
    
    let mut serial = arduino_hal::default_serial!(dp, pins, 57600);
    ufmt::uwriteln!(&mut serial, "Hello, serial!").void_unwrap();

    // Digital pin 13 is also connected to an onboard LED marked "L"
    let mut led = pins.d13.into_output();
    led.set_high();

    loop {
        led.toggle();
        arduino_hal::delay_ms(100);
    }
}

// This can be expanded using eg https://github.com/Rahix/avr-hal/blob/main/examples/arduino-uno/src/bin/uno-panic.rs
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
