//! I2C (Inter-Integrated Circuit) interface for the ferropin crate.
//!
//! This module provides a hardware-agnostic abstraction for I2C communication, allowing
//! interaction with I2C devices on Linux systems.

use crate::error::Result;

/// Bit-banged I2C implementation
pub mod bitbang;
/// Hardware I2C implementation (using /dev/i2c-*)
pub mod hardware;

/// Trait defining the interface for I2C communication
///
/// All I2C implementations should implement this trait to provide
/// a consistent interface for interacting with I2C devices.
pub trait I2c {
    /// Write data to an I2C device
    ///
    /// # Arguments
    ///
    /// * `addr` - The 7-bit I2C address of the device (shifted left by 1, so 0x70 becomes 0xE0)
    /// * `data` - Slice of bytes to write to the device
    ///
    /// # Returns
    ///
    /// A Result indicating success or failure
    fn write(&mut self, addr: u8, data: &[u8]) -> Result<()>;

    /// Read data from an I2C device
    ///
    /// # Arguments
    ///
    /// * `addr` - The 7-bit I2C address of the device (shifted left by 1)
    /// * `buf` - Buffer to store the read data
    ///
    /// # Returns
    ///
    /// A Result indicating success or failure
    fn read(&mut self, addr: u8, buf: &mut [u8]) -> Result<()>;

    /// Write data to then read data from an I2C device (without releasing the bus)
    ///
    /// # Arguments
    ///
    /// * `addr` - The 7-bit I2C address of the device (shifted left by 1)
    /// * `write` - Slice of bytes to write to the device
    /// * `read` - Buffer to store the read data
    ///
    /// # Returns
    ///
    /// A Result indicating success or failure
    fn write_read(&mut self, addr: u8, write: &[u8], read: &mut [u8]) -> Result<()>;
}
