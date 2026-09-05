#![no_std]
#![no_main]

use bao1x_pac as pac;
use embedded_hal::delay::DelayNs;
use panic_halt as _;
use riscv::delay::McycleDelay;

#[riscv_rt::entry]
fn main() -> ! {
    let p = pac::Peripherals::take().unwrap();

    let mut delay = McycleDelay::new(350_000_000);

    // Set GPIO PC8 as output
    p.iox
        .sfr_gpiooe_crgoe2()
        .write(|w| unsafe { w.crgoe2().bits(1 << 8) });

    loop {
        // Set PC8 high
        p.iox
            .sfr_gpioout_crgo2()
            .modify(|r, w| unsafe { w.crgo2().bits(r.crgo2().bits() | (1 << 8)) });

        delay.delay_ms(1000);

        // Set PC8 low
        p.iox
            .sfr_gpioout_crgo2()
            .modify(|r, w| unsafe { w.crgo2().bits(r.crgo2().bits() & !(1 << 8)) });

        delay.delay_ms(1000);
    }
}
