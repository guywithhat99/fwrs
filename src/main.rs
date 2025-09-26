#![no_main]
#![no_std]

use imxrt_hal as hal;
use imxrt_ral as ral;
use ral::gpio::GPIO2;
use ral::iomuxc::IOMUXC;
use cortex_m::asm;

mod fcb;

const CPU_HZ: u32 = 600_000_000;

//entry macro
use imxrt_rt::entry;
use panic_halt as _;

#[entry]
fn main() -> ! {
    // Safety: we're the only code that "owns" the IOMUXC and GPIO2 peripherals.
    let iomuxc = unsafe { IOMUXC::instance() };
    let gpio2 = unsafe { GPIO2::instance() };

    let mut gpio2 = hal::gpio::Port::new(gpio2);
    let pads = hal::iomuxc::into_pads(iomuxc);

    let led = gpio2.output(pads.gpio_b0.p03);
    loop {
        led.toggle();
        asm::delay(CPU_HZ);
    }
}
