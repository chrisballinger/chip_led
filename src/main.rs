extern crate chip_gpio;
extern crate sysfs_gpio;
use sysfs_gpio::{Direction, Result, Pin};
use std::thread::sleep;
use std::time::Duration;

use chip_gpio::ChipPin::*;

fn main() {
    match blink_pins() {
        Ok(_) => println!("Boop"),
        Err(err) => println!("Error: {}", err),
    }
}

fn reset_pins(pins: &Vec<Pin>) -> Result<()>  {
    for pin in pins.iter() {
        pin.with_exported(|| {
            pin.set_direction(Direction::Out)?;
            pin.set_value(0)?;
            Ok(())
        })?;
    }
    Ok(())
}

fn blink_pins() -> Result<()> {
    let red = XIO_P0;
    let yellow = XIO_P2;
    let green = XIO_P4;
    let pins = vec![red.get(), yellow.get(), green.get()];

    println!("Let's blink the pins: {:?}", pins);

    // setup pins
    match reset_pins(&pins) {
        Ok(_) => println!("Pins reset."),
        Err(err) => println!("Error resetting pins: {}", err),
    }
    
    loop {
        for pin in pins.iter() {
            pin.with_exported(|| {
                println!("Pin {:?}", pin);
                pin.set_value(0).unwrap();
                println!("pin {:?}: OFF", pin);
                sleep(Duration::from_millis(500));
                pin.set_value(1).unwrap();
                println!("pin {:?}: ON", pin);
                sleep(Duration::from_millis(500));
                Ok(())
            })?;
        }
    }
}
