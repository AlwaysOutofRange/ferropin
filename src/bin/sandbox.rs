use ferropin::{gpio::{Direction, chardev::ChardevPin}, i2c::{I2c, bitbang::BitbangI2c}};

const CHIP_PATH: &str = "/dev/gpiochip0";

fn main() {
    if let Err(e) = run() {
        eprintln!("{}", e);
        std::process::exit(1);
    }
}

fn run() -> ferropin::error::Result<()>  {
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
