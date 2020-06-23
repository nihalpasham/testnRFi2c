#![no_std]
#![no_main]

extern crate panic_halt;
extern crate nrf52840_hal as hal;

use cortex_m_rt::{entry, exception};
use hal::gpio::{p0, p1};
use hal::twim::{self, Twim};
use hal::target::Peripherals;

#[allow(non_snake_case)]
mod pins_redefined;

use pins_redefined::Pins;
use cortex_m_semihosting::hprintln;

pub const ADDRESS: u8 = 0x60; 

pub fn wake(i2c: &mut Twim<hal::pac::TWIM1>) -> () {
    hprintln!("entering wake").unwrap();
    let bytes = [0; 2];
    let response_to_write = match i2c.write(ADDRESS, &bytes) {
        Ok(v)  => v,
        Err(_e) => panic!("error"),
    }; 
    hprintln!("exiting wake").unwrap();
    return response_to_write
}


#[entry]
fn main () -> ! {
    let p = Peripherals::take().unwrap();
    let pins = Pins::new(p0::Parts::new(p.P0), p1::Parts::new(p.P1));
    
    let scl = pins.p27.into_floating_input().degrade();
    let sda = pins.p26.into_floating_input().degrade();

    let i2c_pins = twim::Pins {scl, sda};

    let mut i2c = Twim::new(p.TWIM1, i2c_pins, twim::Frequency::K100);

    let response = wake(&mut i2c);

    hprintln!("response from slave: {:?}", response).unwrap();

    loop {
         
    }
}

#[exception]
fn HardFault(ef: &cortex_m_rt::ExceptionFrame) -> ! {
    panic!("HardFault at {:#?}", ef);
}

#[exception]
fn DefaultHandler(irqn: i16) {
    panic!("Unhandled exception (IRQn = {})", irqn);
}