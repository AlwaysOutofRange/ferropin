//! GPIO character device interface for the ferropin crate.
//!
//! This module provides an implementation of the `GpioPin` trait using the Linux
//! GPIO character device interface (`/dev/gpiochip*`).
//!
//! The character device interface is the modern way to access GPIO pins on Linux
//! systems, providing more flexibility and better performance than the older sysfs
//! interface.
//!
//! # Usage
//!
//! To use this implementation, create a `ChardevPin` instance by specifying:
//!
//! * The path to the GPIO chip device (usually `/dev/gpiochip0`)
//! * The pin number on that chip
//! * The desired direction (input or output)
//!
//! ```
//! use ferropin::gpio::{Direction, chardev::ChardevPin};
//! use ferropin::error::Result;
//!
//! fn blink_led() -> Result<()> {
//!     // Open GPIO pin 18 as an output
//!     let mut led = ChardevPin::new("/dev/gpiochip0", 18, Direction::Output)?;
//!
//!     // Blink the LED 5 times
//!     for _ in 0..5 {
//!         led.set_high()?;
//!         std::thread::sleep(std::time::Duration::from_millis(500));
//!         led.set_low()?;
//!         std::thread::sleep(std::time::Duration::from_millis(500));
//!     }
//!
//!     Ok(());
//! }
//!
//! # How it works
//!
//! This implementation uses the Linux GPIO character device API, which involves:
//!
//! 1. Opening the GPIO chip device file (e.g., `/dev/gpiochip0`)
//! 2. Using ioctl() calls to request a line handle for a specific pin
//! 3. Using additional ioctl() calls to set/get pin values and configure direction
//!
//! The implementation properly manages resources by closing file descriptors when
//! the `ChardevPin` is dropped.
//!
use std::{
    fs::OpenOptions,
    os::fd::{AsRawFd, RawFd},
};

use crate::{
    err,
    error::{ErrorKind, Result},
    gpio::{Direction, GpioPin},
    sys_utils, try_io,
};

/// Maximum size for GPIO line consumer labels
const GPIO_MAX_NAME_SIZE: usize = 32;
/// Maximum number of handles that can be requested in a single ioctl call
const GPIOHANDLES_MAX: usize = 64;
/// Flag for requesting a line as input
const GPIOHANDLE_REQUEST_INPUT: u32 = 1 << 0;
/// Flag for requesting a line as output
const GPIOHANDLE_REQUEST_OUTPUT: u32 = 1 << 1;
/// Ioctl request to get a line handle
const GPIO_GET_LINEHANDLE_IOCTL: u64 = 0xC16CB403;
/// Ioctl request to get line values
const GPIOHANDLE_GET_LINE_VALUES_IOCTL: u64 = 0xC040B408;
/// Ioctl request to set line values
const GPIOHANDLE_SET_LINE_VALUES_IOCTL: u64 = 0xC040B409;

/// Request structure for GPIO line handle ioctl
#[repr(C)]
struct GpiohandleRequest {
    /// Offsets of the lines to request (relative to the chip)
    lineoffsets: [u32; GPIOHANDLES_MAX],
    /// Flags for the request (input/output)
    flags: u32,
    /// Default values for the lines (for output lines)
    default_values: [u8; GPIOHANDLES_MAX],
    /// Consumer label (identifies what is using the line)
    consumer_label: [u8; GPIO_MAX_NAME_SIZE],
    /// Number of lines being requested
    lines: u32,
    /// File descriptor for the line handle (output parameter)
    fd: i32,
}

/// Data structure for GPIO line values ioctl
#[repr(C)]
struct GpiohandleData {
    /// Values of the lines (0 or 1 for each line)
    values: [u8; GPIOHANDLES_MAX],
}

/// A GPIO pin implemented using the Linux character device interface
///
/// This struct represents a single GPIO pin accessed via the character device
/// interface. It implements the `GpioPin` trait, providing methods to set,
/// read, and configure the pin.
pub struct ChardevPin {
    /// Path to the GPIO chip device file (e.g., "/dev/gpiochip0")
    chip_path: String,
    /// File descriptor for the line handle
    line_fd: RawFd,
    /// The pin number on the chip
    pin: u8,
    /// Current direction of the pin (input or output)
    direction: Direction,
}

impl ChardevPin {
    /// Create a new ChardevPin instance
    ///
    /// # Arguments
    ///
    /// * `chip` - Path to the GPIO chip device (e.g., "/dev/gpiochip0")
    /// * `pin` - The pin number on the chip
    /// * `direction` - Initial direction for the pin (input or output)
    ///
    /// # Returns
    ///
    /// A Result containing the new ChardevPin instance or an error
    ///
    /// # Example
    ///
    /// ```
    /// use ferropin::gpio::{Direction, chardev::ChardevPin};
    /// use ferropin::error::Result;
    ///
    /// let pin = ChardevPin::new("/dev/gpiochip0", 18, Direction::Output)?;
    /// # Ok::<(), ferropin::error::Error>(pin)
    /// ```
    pub fn new(chip: &str, pin: u8, direction: Direction) -> Result<Self> {
        let line_fd = Self::request_line(chip, pin, direction)?;
        Ok(ChardevPin {
            chip_path: chip.to_string(),
            line_fd,
            pin,
            direction,
        })
    }

    /// Request a line handle for a specific pin on a GPIO chip
    ///
    /// # Arguments
    ///
    /// * `chip` - Path to the GPIO chip device
    /// * `pin` - The pin number on the chip
    /// * `direction` - Desired direction for the pin
    ///
    /// # Returns
    ///
    /// A Result containing the file descriptor for the line handle or an error
    fn request_line(chip: &str, pin: u8, direction: Direction) -> Result<RawFd> {
        // Open the GPIO chip device file
        let chip_file = try_io!(OpenOptions::new().read(true).write(true).open(chip));

        // Get the file descriptor for the chip device
        let chip_fd = chip_file.as_raw_fd();

        // Prepare the consumer label (identifies our driver)
        let mut label = [0u8; GPIO_MAX_NAME_SIZE];
        let name = b"ferropin";
        label[..name.len()].copy_from_slice(name);

        // Determine flags based on requested direction
        let flags = match direction {
            Direction::Input => GPIOHANDLE_REQUEST_INPUT,
            Direction::Output => GPIOHANDLE_REQUEST_OUTPUT,
        };

        // Prepare the ioctl request
        let mut req = GpiohandleRequest {
            lineoffsets: [0u32; GPIOHANDLES_MAX],
            flags,
            default_values: [0u8; GPIOHANDLES_MAX],
            consumer_label: label,
            lines: 1,
            fd: -1,
        };
        // Specify which pin we want (offset 0 in our single-pin request)
        req.lineoffsets[0] = pin as u32;

        // Send the ioctl request to get a line handle
        let ret = sys_utils::ioctl(
            chip_fd,
            GPIO_GET_LINEHANDLE_IOCTL,
            &mut req as *mut GpiohandleRequest as u64,
        );

        // Check for errors
        if ret < 0 || req.fd < 0 {
            return Err(err!(ErrorKind::InvalidPin(pin)));
        }

        // Return the file descriptor for the line handle
        Ok(req.fd)
    }

    /// Write a value to the GPIO line
    ///
    /// # Arguments
    ///
    /// * `value` - The value to write (0 for low, non-zero for high)
    ///
    /// # Returns
    ///
    /// A Result indicating success or failure
    fn write_value(&self, value: u8) -> Result<()> {
        // Prepare the data structure for the ioctl call
        let mut data = GpiohandleData {
            values: [0u8; GPIOHANDLES_MAX],
        };
        // Set the value for our single line
        data.values[0] = value;

        // Send the ioctl request to set the line values
        let ret = sys_utils::ioctl(
            self.line_fd,
            GPIOHANDLE_SET_LINE_VALUES_IOCTL,
            &mut data as *mut GpiohandleData as u64,
        );
        // Check for errors
        if ret < 0 {
            return Err(err!(ErrorKind::Io(std::io::Error::last_os_error())));
        }

        Ok(())
    }
}

impl GpioPin for ChardevPin {
    /// Set the pin to a high voltage level
    ///
    /// # Returns
    ///
    /// A Result indicating success or failure
    fn set_high(&mut self) -> Result<()> {
        self.write_value(1)
    }

    /// Set the pin to a low voltage level (ground)
    ///
    /// # Returns
    ///
    /// A Result indicating success or failure
    fn set_low(&mut self) -> Result<()> {
        self.write_value(0)
    }

    /// Read the current value of the pin
    ///
    /// # Returns
    ///
    /// A Result containing `true` if the pin is high, `false` if low, or an error
    fn read(&self) -> Result<bool> {
        // Prepare the data structure for the ioctl call
        let mut data = GpiohandleData {
            values: [0u8; GPIOHANDLES_MAX],
        };

        // Send the ioctl request to get the line values
        let ret = sys_utils::ioctl(
            self.line_fd,
            GPIOHANDLE_GET_LINE_VALUES_IOCTL,
            &mut data as *mut GpiohandleData as u64,
        );
        // Check for errors
        if ret < 0 {
            return Err(err!(ErrorKind::Io(std::io::Error::last_os_error())));
        }

        // Return true if the value is non-zero (high), false if zero (low)
        Ok(data.values[0] != 0)
    }

    /// Set the direction of the pin (input or output)
    ///
    /// # Arguments
    ///
    /// * `direction` - The desired direction for the pin
    ///
    /// # Returns
    ///
    /// A Result indicating success or failure
    ///
    /// # Note
    ///
    /// Changing the direction requires closing the current line handle and
    /// requesting a new one with the appropriate flags.
    fn set_direction(&mut self, direction: Direction) -> Result<()> {
        // If the direction is already correct, do nothing
        if self.direction == direction {
            return Ok(());
        }

        // Close the current line handle
        sys_utils::close(self.line_fd);

        // Request a new line handle with the new direction
        self.line_fd = Self::request_line(&self.chip_path, self.pin, direction)?;
        // Update our stored direction
        self.direction = direction;

        Ok(())
    }
}

impl Drop for ChardevPin {
    /// Close the line handle file descriptor when the ChardevPin is dropped
    fn drop(&mut self) {
        sys_utils::close(self.line_fd);
    }
}
