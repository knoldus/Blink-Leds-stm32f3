#![no_std]
#![no_main]

use blink_stm32::config::initialization::{mycrate,entry, OutputSwitch};
use stm32f3_discovery::stm32f3xx_hal::prelude::*;

/// This program is going to blink all the LEDs of stm32f3 board with 1 sec of delay.
///
/// This is starting point of the no_main program.
#[entry]
fn main() -> ! {
    let mut counter = 0;
    let (leds, mut delay) = mycrate();
    let mut led = leds.into_array();
    loop {
        while counter < counter + 1 {
            let next = counter % 8;
            led[next].on().ok();
            delay.delay_ms(1000_u16);
            led[next].off().ok();
            counter = counter + 1;
        }
    }
}
