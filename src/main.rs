extern crate chip_gpio;
extern crate sysfs_gpio;
extern crate ctrlc;

use sysfs_gpio::{Direction, Result, Pin};
use std::thread::sleep;
use std::time::Duration;

use chip_gpio::ChipPin::*;

use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};

fn main() {
    let red = XIO_P0;
    let yellow = XIO_P2;
    let green = XIO_P4;
    let pins = vec![red.get(), yellow.get(), green.get()];

    let running = Arc::new(AtomicBool::new(true));
 
    setup_exit_handler(running.clone());

    // setup pins
    match reset_pins(&pins) {
        Ok(_) => println!("Pins reset."),
        Err(err) => println!("Error resetting pins: {}", err),
    }

    println!("Let's blink the pins: {:?}", pins);
    while running.load(Ordering::SeqCst) {
        match blink_pins(&pins) {
            Ok(_) => (),
            Err(err) => println!("Error blinking pins: {}", err)
        }
    }
    reset_pins(&pins).unwrap();
    println!("Bye!");
}

fn setup_exit_handler(running: std::sync::Arc<std::sync::atomic::AtomicBool>) {
    ctrlc::set_handler(move || {
        println!("Quitting...");
        running.store(false, Ordering::SeqCst);
    }).expect("Error setting Ctrl-C handler");
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

fn blink_pin(pin: &Pin, duration: u64) -> Result<()> {
    pin.with_exported(|| {
        pin.set_value(0)?;
        println!("pin {:?}: OFF", pin);
        sleep(Duration::from_millis(duration));
        pin.set_value(1)?;
        println!("pin {:?}: ON", pin);
        sleep(Duration::from_millis(duration));
        Ok(())
    })?;
    Ok(())
}

fn blink_pins(pins: &Vec<Pin>) -> Result<()> {
    for pin in pins.iter() {
        blink_pin(pin, 100)?;
    }
    Ok(())
}
