#![no_main]
#![no_std]

//The major compnents of embedded rust needed to run on microcontroller
mod fcb;
use teensy4_bsp::{self as bsp, rt::entry};
use teensy4_panic as _;



//GUARDS! Bring in the HAL and RAL
use imxrt_hal as hal;
use imxrt_ral as ral;

//bring the board into scope
use bsp::board;

//used for delay fuction
use cortex_m::asm;

//used for usb
use usb_device::prelude::*;
use usbd_serial::{SerialPort, USB_CLASS_CDC};
use hal::usbd;  

// Static buffers required by the i.MX RT USB device driver
static ENDPOINT_MEMORY: usbd::EndpointMemory<2048> = usbd::EndpointMemory::new();
static ENDPOINT_STATE: usbd::EndpointState = usbd::EndpointState::max_endpoints();

//entry macro
#[entry]
fn main() -> ! {
    // Prepare the USB clocks.
    let mut ccm_analog = unsafe { ral::ccm_analog::CCM_ANALOG::instance() };
    hal::ccm::analog::pll3::restart(&mut ccm_analog);

    //get the instances of teensy hardware and assigns them into the resoruces stuct in the board module
    // ".." just says ignore everything else, when we need more peripherals, we can explictly bind them here
    let board::Resources {
        pins,
        mut gpio2,
        usb,
        ..
    } = board::t41(board::instances());


    
    //create the usb bus
    let bus = usbd::BusAdapter::new(usb, &ENDPOINT_MEMORY, &ENDPOINT_STATE);
    let bus_allocator = usb_device::bus::UsbBusAllocator::new(bus);



    //create the CDC-ACM class and device descriptor
    let mut serial = SerialPort::new(&bus_allocator);

    let mut device = UsbDeviceBuilder::new(&bus_allocator, UsbVidPid(0x16C0, 0x0483))
        .product("Teensy 4.1")
        .serial_number("T41-0001")
        .build();

        let led = board::led(&mut gpio2, pins.p13);
    loop {
        
        led.toggle();
        //poll until USB configured
        if device.poll(&mut []) {
            let state = device.state();
            if state == usb_device::device::UsbDeviceState::Configured {
                break;
            }
        }
    }

    device.bus().configure();


    let mut buf = [0u8; 64];


    
    loop {
        
        if !device.poll(&mut [&mut serial]) {
            continue;
        }

         


        let _ = serial.write(b"hello, world\r\n");


       
        asm::delay(600000000);
    }
}
