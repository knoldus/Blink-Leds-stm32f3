#![no_std]
#![no_main]
/// This program is going to blink all the LEDs of stm32f3 board with 1 sec of delay.
///
/// Function mycrate() contains the implementation to cover the led challenge of discovery book.
use my_auxillary::*;
/// This is starting point of the no_main program.
#[entry]
fn main() -> ! {
    let mut counter = 0;
    let (mut led, mut delay) = mycrate();

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