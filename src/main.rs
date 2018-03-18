extern crate chip_gpio;
extern crate sysfs_gpio;
use sysfs_gpio::{Direction, Pin};
use std::thread::sleep;
use std::time::Duration;

use chip_gpio::ChipPin::*;

fn main() {
    println!("Let's blink the LED!");
    let my_led = XIO_P0.get();
    my_led.with_exported(|| {
        loop {
            my_led.set_value(0).unwrap();
            sleep(Duration::from_millis(200));
            my_led.set_value(1).unwrap();
            sleep(Duration::from_millis(200));
        }
    }).unwrap();
}
