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
    //get the instances of teensy hardware and assigns them into the resoruces stuct in the board module
    // ".." just says ignore everything else, when we need more peripherals, we can explictly bind them here
    let board::Resources {pins, mut gpio2, ..} = board::t41(board::instances());

    let led= board::led(&mut gpio2, pins.p13);

    loop {
        led.toggle();
        asm::delay(600000000);



    }
}
