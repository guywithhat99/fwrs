#![no_std]
#![no_main]

mod comms;
mod controls;
mod devices;
mod utils;

use cortex_m::asm;
use cortex_m_rt::entry;

use panic_halt as _; // panic handler... just halts

#[entry]
fn main() -> ! {
    // init everything
    loop {
        asm::nop();
    }
}
