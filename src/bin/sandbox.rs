#![allow(dead_code)]
use std::{thread::sleep, time::Duration};

use ferropin::{
    display::ssd1306::Ssd1306,
    gpio::{Direction, chardev::ChardevPin},
    i2c::{I2c, bitbang::BitbangI2c, hardware::HardwareI2c},
};

const CHIP_PATH: &str = "/dev/gpiochip0";

fn main() {
    // TODO: Fix this chardev shit
    // if let Err(e) = chardev_test() {
    //     eprintln!("{}", e);
    //     std::process::exit(1);
    // }
    //

    if let Err(e) = display_test_via_hardwarei2c() {
        eprintln!("{}", e);
        std::process::exit(1);
    }
}

fn chardev_test() -> ferropin::error::Result<()> {
    println!("Opening GPIO 2 (SDA) and GPIO 3 (SCL) via chardev");

    // This is the "data line"
    let sda = ChardevPin::new(CHIP_PATH, 2, Direction::Output)?;
    // This is the "clock line"
    let scl = ChardevPin::new(CHIP_PATH, 3, Direction::Output)?;

    let mut i2c = BitbangI2c::new(sda, scl);

    // Send a single byte to the SSD1306 address (0x3C)
    // 0x00 = control byte meaning "this is a command"
    // 0xAF = display ON command
    println!("Sending display ON command...");
    i2c.write(0x3C, &[0x00, 0xAF])?;
    println!("ACK received — display is alive!");

    Ok(())
}


fn display_test_via_hardwarei2c() -> ferropin::error::Result<()> {
    let i2c = HardwareI2c::new(1)?;
    let mut display = Ssd1306::new(i2c)?;

    // Test 1: single pixel top-left
    display.clear();
    display.set_pixel(0, 0, true);
    display.flush()?;
    sleep(Duration::from_secs(3));

    // Test 2: single pixel placed in the center
    display.clear();
    display.set_pixel(64, 32, true);
    display.flush()?;
    sleep(Duration::from_secs(3));

    // Test 3: fill just the first page (rows 0-7)
    display.clear();
    for x in 0..128 {
        for y in 0..8 {
            display.set_pixel(x, y, true);
        }
    }
    display.flush()?;
    sleep(Duration::from_secs(3));

    // Test 4: fill just the first column
    display.clear();
    for y in 0..64 {
        display.set_pixel(0, y, true);
    }
    display.flush()?;
    sleep(Duration::from_secs(3));

    Ok(())
}
