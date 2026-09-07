#![no_std]
#![no_main]

use bao1x_hal as hal;
use embedded_hal::{delay::DelayNs, digital::OutputPin};
use hal::pac;
use panic_halt as _;
use riscv::delay::McycleDelay;

#[riscv_rt::entry]
fn main() -> ! {
    let p = pac::Peripherals::take().unwrap();

    let pins = hal::gpio::Pins::new(p.iox);
    let mut led = pins.pc08.into_output();

    let mut delay = McycleDelay::new(350_000_000);

    loop {
        led.set_high();

        delay.delay_ms(1000);

        led.set_low();

        delay.delay_ms(1000);
    }
}
