#![no_main]
#![no_std]

//The major compnents of embedded rust needed to run on microcontroller
mod fcb;
use teensy4_bsp::{self as bsp, rt::entry};
use teensy4_panic as _;

//bring the board into scope
use bsp::board;

//used for delay fuction
use cortex_m::asm;


//entry macro
#[entry]
fn main() -> ! {
    let board::Resources {pins, mut gpio2, ..} = board::t41(board::instances());

    let led= board::led(&mut gpio2, pins.p13);

    loop {
        led.toggle();
        asm::delay(600000000);



    }
}
