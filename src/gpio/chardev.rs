//! GPIO character device interface for the ferropin crate.
//!
//! This module provides an implementation of the `GpioPin` trait using the Linux
//! GPIO character device interface (`/dev/gpiochip*`).

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
    /// # Ok::<(), ferropin::error::Error>(())
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
    fn request_line(chip: &str, pin: u8, direction: Direction) -> Result<RawFd> {
        let chip_file = try_io!(OpenOptions::new().read(true).write(true).open(chip));
        let chip_fd = chip_file.as_raw_fd();

        let mut label = [0u8; GPIO_MAX_NAME_SIZE];
        let name = b"ferropin";
        label[..name.len()].copy_from_slice(name);

        let flags = match direction {
            Direction::Input => GPIOHANDLE_REQUEST_INPUT,
            Direction::Output => GPIOHANDLE_REQUEST_OUTPUT,
        };

        let mut req = GpiohandleRequest {
            lineoffsets: [0u32; GPIOHANDLES_MAX],
            flags,
            default_values: [0u8; GPIOHANDLES_MAX],
            consumer_label: label,
            lines: 1,
            fd: -1,
        };
        req.lineoffsets[0] = pin as u32;

        let ret = sys_utils::ioctl(
            chip_fd,
            GPIO_GET_LINEHANDLE_IOCTL,
            &mut req as *mut GpiohandleRequest as u64,
        );

        if ret < 0 || req.fd < 0 {
            return Err(err!(ErrorKind::InvalidPin(pin)));
        }

        Ok(req.fd)
    }

    /// Write a value to the GPIO line
    fn write_value(&self, value: u8) -> Result<()> {
        let mut data = GpiohandleData {
            values: [0u8; GPIOHANDLES_MAX],
        };
        data.values[0] = value;

        let ret = sys_utils::ioctl(
            self.line_fd,
            GPIOHANDLE_SET_LINE_VALUES_IOCTL,
            &mut data as *mut GpiohandleData as u64,
        );
        if ret < 0 {
            return Err(err!(ErrorKind::Io(std::io::Error::last_os_error())));
        }

        Ok(())
    }
}

impl GpioPin for ChardevPin {
    /// Set the pin to a high voltage level
    fn set_high(&mut self) -> Result<()> {
        self.write_value(1)
    }

    /// Set the pin to a low voltage level (ground)
    fn set_low(&mut self) -> Result<()> {
        self.write_value(0)
    }

    /// Read the current value of the pin
    fn read(&self) -> Result<bool> {
        let mut data = GpiohandleData {
            values: [0u8; GPIOHANDLES_MAX],
        };

        let ret = sys_utils::ioctl(
            self.line_fd,
            GPIOHANDLE_GET_LINE_VALUES_IOCTL,
            &mut data as *mut GpiohandleData as u64,
        );
        if ret < 0 {
            return Err(err!(ErrorKind::Io(std::io::Error::last_os_error())));
        }

        Ok(data.values[0] != 0)
    }

    /// Set the direction of the pin (input or output)
    fn set_direction(&mut self, direction: Direction) -> Result<()> {
        if self.direction == direction {
            return Ok(());
        }

        sys_utils::close(self.line_fd);

        self.line_fd = Self::request_line(&self.chip_path, self.pin, direction)?;
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
