//! Bit-banged I2C implementation for the ferropin crate.
//!
//! This module provides a software-based I2C implementation that uses GPIO pins to
//! simulate I2C communication.

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

/// Bit-banged I2C implementation using GPIO pins
pub struct BitbangI2c<P: GpioPin> {
    /// Data line (SDA)
    sda: P,
    /// Clock line (SCL)
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
    fn stop(&mut self) -> Result<()> {
        self.sda.set_low()?;
        self.scl.set_high()?;
        sleep(HALF_CYCLE);
        self.sda.set_high()?;
        sleep(HALF_CYCLE);
        Ok(())
    }

    /// Send one bit over the I2C bus
    fn write_bit(&mut self, bit: bool) -> Result<()> {
        self.scl.set_low()?;
        sleep(HALF_CYCLE);
        self.sda.set(bit)?;
        self.scl.set_high()?;
        sleep(HALF_CYCLE);
        Ok(())
    }

    /// Read one bit from the I2C bus
    fn read_bit(&mut self) -> Result<bool> {
        self.scl.set_low()?;
        sleep(HALF_CYCLE);
        self.sda.set_direction(Direction::Input)?;
        self.scl.set_high()?;
        sleep(HALF_CYCLE);
        let bit = self.sda.read()?;
        self.sda.set_direction(Direction::Output)?;
        Ok(bit)
    }

    /// Send one byte and read the ACK/NACK response
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

    fn write_read(&mut self, addr: u8, write: &[u8], read: &mut [u8]) -> Result<()> {
        self.write(addr, write)?;
        self.read(addr, read)?;

        Ok(())
    }
}
