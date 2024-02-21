#![no_std]
#![no_main]

use esp32c3_hal::{clock::ClockControl, peripherals::Peripherals, prelude::*, Delay, gpio::IO};
use esp_backtrace as _;
use esp_println::println;

use dht_sensor::*;

#[entry]
fn main() -> ! {
    let per = Peripherals::take();
    let system = per.SYSTEM.split();

    let clocks = ClockControl::max(system.clock_control).freeze();
    let mut delay = Delay::new(&clocks);

    let io = IO::new(per.GPIO, per.IO_MUX);

    let mut pin = io.pins.gpio19.into_open_drain_output();
    let temperature = dht_sensor::dht22::Reading::read(&mut delay, &mut pin);

    println!("Hello world!");
    loop {
        println!("Loop...{:?}", temperature);
        delay.delay_ms(500u32);
    }
}
