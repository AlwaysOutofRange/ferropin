//! Bit-banged I2C implementation for the ferropin crate.
//!
//! This module provides a software-based I2C implementation that uses GPIO pins to
//! simulate I2C communication. This is useful when hardware I2C is not available
//! or when you need to communicate with I2C devices on arbitrary GPIO pins.
//!
//! # Usage
//!
//! Create a `BitbangI2c` instance by providing two GPIO pins (SDA and SCL):
//!
//! ```
//! use ferropin::gpio::{Direction, chardev::ChardevPin};
//! use ferropin::i2c::bitbang::BitbangI2c;
//! use ferropin::error::Result;
//!
//! fn main() -> Result<()> {
//!     // Initialize GPIO pins for SDA and SCL
//!     let sda = ChardevPin::new("/dev/gpiochip0", 2, Direction::Output)?;
//!     let scl = ChardevPin::new("/dev/gpiochip0", 3, Direction::Output)?;
//!
//!     // Create bit-banged I2C interface
//!     let mut i2c = BitbangI2c::new(sda, scl);
//!
//!     // Use the I2C interface
//!     i2c.write(0x40, &[0x00, 0xFF])?;
//!
//!     let mut buffer = [0u8; 2];
//!     i2c.read(0x40, &mut buffer)?;
//!
//!     Ok(())
//! }
//! ```
//!
//! # How it works
//!
//! This implementation:
//! 1. Uses two GPIO pins: one for data (SDA) and one for clock (SCL)
//! 2. Implements I2C protocol using software timing
//! 3. Operates at standard mode (100kHz)
//! 4. Supports start/stop conditions, bit reading/writing, and ACK/NACK
//!
use std::{thread::sleep, time::Duration};

use crate::{
    err,
    error::{ErrorKind, Result},
    gpio::{Direction, GpioPin},
    i2c::I2c,
};

// I2C clock speed — standard mode is 100kHz
// Each half-cycle is 5µs → full cycle = 10µs = 100kHz
const HALF_CYCLE: Duration = Duration::from_micros(5);

//       START        BIT=1       BIT=0        STOP
// SDA:  ‾‾‾\____    ‾‾‾‾‾‾‾‾    ________    ____/‾‾‾
//
// SCL:  ‾‾‾‾‾‾‾‾    ___‾‾‾___   ___‾‾‾___   ‾‾‾‾‾‾‾‾

/// Bit-banged I2C implementation using GPIO pins
pub struct BitbangI2c<P: GpioPin> {
    // Data Line
    sda: P,
    // Clock Line,
    scl: P,
}

impl<P: GpioPin> BitbangI2c<P> {
    /// Create a new bit-banged I2C instance
    ///
    /// # Arguments
    ///
    /// * `sda` - GPIO pin for the data line (SDA)
    /// * `scl` - GPIO pin for the clock line (SCL)
    ///
    /// # Returns
    ///
    /// A new BitbangI2c instance
    pub fn new(sda: P, scl: P) -> Self {
        BitbangI2c { sda, scl }
    }

    /// Send I2C start condition
    ///
    /// Start condition: SDA goes low while SCL is high
    fn start(&mut self) -> Result<()> {
        self.sda.set_high()?;

        self.scl.set_high()?;
        sleep(HALF_CYCLE);

        self.sda.set_low()?;

        self.scl.set_low()?;
        sleep(HALF_CYCLE);

        Ok(())
    }

    /// Send I2C stop condition
    ///
    /// Stop condition: SDA goes high while SCL is high
    fn stop(&mut self) -> Result<()> {
        self.sda.set_low()?;

        self.scl.set_high()?;
        sleep(HALF_CYCLE);

        self.sda.set_high()?;
        sleep(HALF_CYCLE);

        Ok(())
    }

    /// Send one bit over the I2C bus
    ///
    /// # Arguments
    ///
    /// * `bit` - The bit to send (true for high, false for low)
    fn write_bit(&mut self, bit: bool) -> Result<()> {
        self.scl.set_low()?;
        sleep(HALF_CYCLE);

        self.sda.set(bit)?;

        self.scl.set_high()?;
        sleep(HALF_CYCLE);

        Ok(())
    }

    /// Read one bit from the I2C bus
    ///
    /// # Returns
    ///
    /// The bit read from the bus (true for high, false for low)
    fn read_bit(&mut self) -> Result<bool> {
        self.scl.set_low()?;
        sleep(HALF_CYCLE);

        // Release SDA so the device can drive it
        self.sda.set_direction(Direction::Input)?;

        self.scl.set_high()?;
        sleep(HALF_CYCLE);

        let bit = self.sda.read()?;

        // Take SDA back
        self.sda.set_direction(Direction::Output)?;

        Ok(bit)
    }

    /// Send one byte and read the ACK/NACK response
    ///
    /// # Arguments
    ///
    /// * `byte` - The byte to send
    ///
    /// # Returns
    ///
    /// true if ACK received, false if NACK received
    fn write_byte(&mut self, byte: u8) -> Result<bool> {
        // Send 8 bits MSB first
        for i in (0..8).rev() {
            self.write_bit((byte >> i) & 1 == 1)?;
        }

        // 9th clock — read ACK from device
        // ACK = SDA pulled LOW by device
        // NACK = SDA stays HIGH
        let nack = self.read_bit()?;
        Ok(!nack)
    }

    /// Read one byte and send ACK/NACK response
    ///
    /// # Arguments
    ///
    /// * `ack` - true to send ACK, false to send NACK
    ///
    /// # Returns
    ///
    /// The byte read from the bus
    fn read_byte(&mut self, ack: bool) -> Result<u8> {
        let mut byte = 0u8;

        for i in (0..8).rev() {
            if self.read_bit()? {
                byte |= 1 << i;
            }
        }

        // Send ACK (LOW) or NACK (HIGH) back to device
        self.write_bit(!ack)?;
        Ok(byte)
    }
}

impl<P: GpioPin> I2c for BitbangI2c<P> {
    /// Write data to an I2C device
    ///
    /// # Arguments
    ///
    /// * `addr` - The 7-bit I2C address of the device
    /// * `data` - Slice of bytes to write to the device
    fn write(&mut self, addr: u8, data: &[u8]) -> Result<()> {
        self.start()?;

        // Send address byte — left shift by 1, LSB=0 means write
        if !self.write_byte((addr << 1) | 0)? {
            self.stop()?;
            return Err(err!(ErrorKind::I2cNack));
        }

        // Send data bytes
        for &byte in data {
            if !self.write_byte(byte)? {
                self.stop()?;
                return Err(err!(ErrorKind::I2cNack));
            }
        }

        self.stop()?;
        Ok(())
    }

    /// Read data from an I2C device
    ///
    /// # Arguments
    ///
    /// * `addr` - The 7-bit I2C address of the device
    /// * `buf` - Buffer to store the read data
    fn read(&mut self, addr: u8, buf: &mut [u8]) -> Result<()> {
        self.start()?;

        // Send address byte - LSB=1 means read
        if !self.write_byte((addr << 1) | 1)? {
            self.stop()?;
            return Err(err!(ErrorKind::I2cNack));
        }

        // Read bytes - ACK all expect the last one
        let last = buf.len() - 1;
        for (i, slot) in buf.iter_mut().enumerate() {
            *slot = self.read_byte(i != last)?;
        }

        self.stop()?;
        Ok(())
    }

    /// Write data to then read data from an I2C device
    ///
    /// # Arguments
    ///
    /// * `addr` - The 7-bit I2C address of the device
    /// * `write` - Slice of bytes to write to the device
    /// * `read` - Buffer to store the read data
    fn write_read(&mut self, addr: u8, write: &[u8], read: &mut [u8]) -> Result<()> {
        self.write(addr, write)?;
        self.read(addr, read)?;

        Ok(())
    }
}
