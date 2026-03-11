use std::{thread::sleep, time::Duration};

use crate::{
    err,
    error::{ErrorKind, Result},
    gpio::{Direction, GpioPin},
    i2c::I2c,
};

const HALF_CYCLE: Duration = Duration::from_micros(5);

#[doc = "Bit-banged I2C implementation using GPIO pins"]
pub struct BitbangI2c<P: GpioPin> {
    sda: P,
    scl: P,
}

impl<P: GpioPin> BitbangI2c<P> {
    #[doc = "Create a new bit-banged I2C instance"]
    pub fn new(sda: P, scl: P) -> Self {
        BitbangI2c { sda, scl }
    }

    fn start(&mut self) -> Result<()> {
        self.sda.set_high()?;
        self.scl.set_high()?;
        sleep(HALF_CYCLE);
        self.sda.set_low()?;
        self.scl.set_low()?;
        sleep(HALF_CYCLE);
        Ok(())
    }

    fn stop(&mut self) -> Result<()> {
        self.sda.set_low()?;
        self.scl.set_high()?;
        sleep(HALF_CYCLE);
        self.sda.set_high()?;
        sleep(HALF_CYCLE);
        Ok(())
    }

    fn write_bit(&mut self, bit: bool) -> Result<()> {
        self.scl.set_low()?;
        sleep(HALF_CYCLE);
        self.sda.set(bit)?;
        self.scl.set_high()?;
        sleep(HALF_CYCLE);
        Ok(())
    }

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

    fn write_byte(&mut self, byte: u8) -> Result<bool> {
        for i in (0..8).rev() {
            self.write_bit((byte >> i) & 1 == 1)?;
        }
        let nack = self.read_bit()?;
        Ok(!nack)
    }

    fn read_byte(&mut self, ack: bool) -> Result<u8> {
        let mut byte = 0u8;

        for i in (0..8).rev() {
            if self.read_bit()? {
                byte |= 1 << i;
            }
        }

        self.write_bit(!ack)?;
        Ok(byte)
    }
}

impl<P: GpioPin> I2c for BitbangI2c<P> {
    fn write(&mut self, addr: u8, data: &[u8]) -> Result<()> {
        self.start()?;

        if !self.write_byte((addr << 1) | 0)? {
            self.stop()?;
            return Err(err!(ErrorKind::I2cNack));
        }

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

        if !self.write_byte((addr << 1) | 1)? {
            self.stop()?;
            return Err(err!(ErrorKind::I2cNack));
        }

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
