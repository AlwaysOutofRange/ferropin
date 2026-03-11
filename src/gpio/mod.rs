//! GPIO (General Purpose Input/Output) interface for the ferropin crate.
//!
//! This module provides a hardware-agnostic abstraction for GPIO pins, allowing
//! interaction with digital input/output pins on Linux systems.

use crate::error::Result;

/// Character device implementation of GPIO pins
pub mod chardev;

/// Represents the direction a GPIO pin can be configured for
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Direction {
    /// Pin configured as input (can read values)
    Input,
    /// Pin configured as output (can write values)
    Output,
}

/// Trait defining the interface for GPIO pins
///
/// All GPIO pin implementations should implement this trait to provide
/// a consistent interface for interacting with GPIO hardware.
pub trait GpioPin {
    /// Set the pin to a high voltage level
    fn set_high(&mut self) -> Result<()>;

    /// Set the pin to a low voltage level (ground)
    fn set_low(&mut self) -> Result<()>;

    /// Read the current value of the pin
    ///
    /// Returns `Ok(true)` if the pin is high, `Ok(false)` if low
    fn read(&self) -> Result<bool>;

    /// Set the direction of the pin (input or output)
    fn set_direction(&mut self, direction: Direction) -> Result<()>;

    /// Set the pin to high or low based on a boolean value
    ///
    /// This is a convenience function that calls `set_high()` if `high` is true,
    /// or `set_low()` if `high` is false.
    fn set(&mut self, high: bool) -> Result<()> {
        if high {
            self.set_high()
        } else {
            self.set_low()
        }
    }
}
