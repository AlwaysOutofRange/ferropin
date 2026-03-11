//! Hardware I2C implementation for the ferropin crate.
//!
//! This module provides a hardware-based I2C implementation using Linux I2C device files
//! (typically `/dev/i2c-*`). This implementation is faster and more reliable than
//! bit-banged I2C but requires hardware I2C support on the Linux system.
//!
//! # Usage
//!
//! Create a `HardwareI2c` instance by specifying the I2C bus number (e.g., 1 for `/dev/i2c-1`):
//!
//! ```
//! use ferropin::i2c::hardware::HardwareI2c;
//! use ferropin::error::Result;
//!
//! fn main() -> Result<()> {
//!     // Open I2C bus 1 (typically /dev/i2c-1)
//!     let mut i2c = HardwareI2c::new(1)?;
//!
//!     // Write and read data
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
//! 1. Opens the I2C device file (e.g., `/dev/i2c-1`)
//! 2. Uses ioctl to set the slave address before each operation
//! 3. Uses low-level system calls for reading/writing data
//! 4. Properly manages file descriptors
//!
use std::os::fd::RawFd;

use crate::{
    err,
    error::{ErrorKind, Result},
    i2c::I2c,
    sys_utils,
};

/// Hardware I2C implementation using Linux I2C device files
pub struct HardwareI2c {
    /// File descriptor for the I2C device file
    fd: RawFd,
    /// Currently set slave address (for optimization)
    current_addr: u8,
}

impl HardwareI2c {
    /// Create a new HardwareI2c instance
    ///
    /// # Arguments
    ///
    /// * `bus` - The I2C bus number (e.g., 1 for `/dev/i2c-1`)
    ///
    /// # Returns
    ///
    /// A Result containing the new HardwareI2c instance or an error
    pub fn new(bus: u8) -> Result<Self> {
        // Build the path to the I2C device file
        let path = format!("/dev/i2c-{}", bus);

        // Open the device file for read and write access
        let fd = sys_utils::open(path.as_ptr(), sys_utils::O_RDWR);
        if fd < 0 {
            return Err(err!(ErrorKind::Io(std::io::Error::last_os_error())));
        }

        Ok(HardwareI2c {
            fd: fd as RawFd,
            current_addr: 0xFF, // Invalid initial address
        })
    }

    /// Set the slave address for subsequent operations
    ///
    /// # Arguments
    ///
    /// * `addr` - The 7-bit I2C address of the device (shifted left by 1)
    ///
    /// # Returns
    ///
    /// A Result indicating success or failure
    fn set_addr(&mut self, addr: u8) -> Result<()> {
        // Skip if address is already set
        if self.current_addr == addr {
            return Ok(());
        }

        // Use ioctl to set the slave address
        let ret = sys_utils::ioctl(self.fd, sys_utils::I2C_SLAVE, addr as u64);
        if ret < 0 {
            return Err(err!(ErrorKind::Io(std::io::Error::last_os_error())));
        }

        self.current_addr = addr;

        Ok(())
    }
}

impl I2c for HardwareI2c {
    fn write(&mut self, addr: u8, data: &[u8]) -> Result<()> {
        self.set_addr(addr)?;

        // Write the data to the I2C device
        let ret = sys_utils::write(self.fd, data.as_ptr(), data.len());
        if ret < 0 {
            return Err(err!(ErrorKind::Io(std::io::Error::last_os_error())));
        }

        Ok(())
    }

    fn read(&mut self, addr: u8, buf: &mut [u8]) -> Result<()> {
        self.set_addr(addr)?;

        // Read data from the I2C device
        let ret = sys_utils::read(self.fd, buf.as_mut_ptr(), buf.len());
        if ret < 0 {
            return Err(err!(ErrorKind::Io(std::io::Error::last_os_error())));
        }

        Ok(())
    }

    fn write_read(&mut self, addr: u8, write: &[u8], read: &mut [u8]) -> Result<()> {
        // Perform write followed by read (without releasing the bus)
        self.write(addr, write)?;
        self.read(addr, read)?;

        Ok(())
    }
}

impl Drop for HardwareI2c {
    /// Close the I2C device file when the HardwareI2c instance is dropped
    fn drop(&mut self) {
        sys_utils::close(self.fd);
    }
}
