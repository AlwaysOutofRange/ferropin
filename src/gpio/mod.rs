//! GPIO (General Purpose Input/Output) interface for the ferropin crate.
//!
//! This module provides a hardware-agnostic abstraction for GPIO pins, allowing
//! interaction with digital input/output pins on Linux systems.
//!
//! # Features
//!
//! * Hardware-agnostic GPIO pin interface
//! * Support for different GPIO implementations (currently character device)
//! * Direction configuration (input/output)
//! * Value reading and setting
//!
//! # Usage
//!
//! The GPIO module provides a trait `GpioPin` that defines the interface for GPIO pins.
//! Different implementations (like the character device implementation) provide concrete
//! types that implement this trait.
//!
//! ## Example using the character device implementation
//!
//! ```
//! use ferropin::gpio::{Direction, chardev::ChardevPin};
//! use ferropin::error::Result;
//!
//! fn toggle_led() -> Result<()> {
//!     // Open GPIO pin 18 as an output
//!     let mut led = ChardevPin::new("/dev/gpiochip0", 18, Direction::Output)?;
//!
//!     // Turn the LED on
//!     led.set_high()?;
//!
//!     // Turn the LED off
//!     led.set_low()?;
//!
//!     Ok(())
//! ```
//!
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
