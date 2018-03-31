extern crate chip_gpio;
extern crate sysfs_gpio;
use sysfs_gpio::{Direction};
use std::thread::sleep;
use std::time::Duration;

use chip_gpio::ChipPin::*;

fn main() {
    blink_pins();
}

fn blink_pins() {
    let red = XIO_P0;
    let yellow = XIO_P2;
    let green = XIO_P4;
    let pins = [red.get(), yellow.get(), green.get()];

    println!("Let's blink the pins: {:?}", pins);

    // setup pins
    for pin in pins.iter() {
        pin.export().unwrap();
        pin.set_direction(Direction::Out).unwrap();
        for pin in pins.iter() {
            pin.set_value(0).unwrap();
        };
    }
    
    loop {
        for pin in pins.iter() {
            println!("Pin {:?}", pin);
            pin.set_value(0).unwrap();
            println!("pin {:?}: OFF", pin);
            sleep(Duration::from_millis(500));
            pin.set_value(1).unwrap();
            println!("pin {:?}: ON", pin);
            sleep(Duration::from_millis(500));
        }
    }
}
