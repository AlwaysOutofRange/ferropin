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

const GPIO_MAX_NAME_SIZE: usize = 32;
const GPIOHANDLES_MAX: usize = 64;
const GPIOHANDLE_REQUEST_INPUT: u32 = 1 << 0;
const GPIOHANDLE_REQUEST_OUTPUT: u32 = 1 << 1;
const GPIO_GET_LINEHANDLE_IOCTL: u64 = 0xC16CB403;
const GPIOHANDLE_GET_LINE_VALUES_IOCTL: u64 = 0xC040B408;
const GPIOHANDLE_SET_LINE_VALUES_IOCTL: u64 = 0xC040B409;

#[repr(C)]
struct GpiohandleRequest {
    lineoffsets: [u32; GPIOHANDLES_MAX],
    flags: u32,
    default_values: [u8; GPIOHANDLES_MAX],
    consumer_label: [u8; GPIO_MAX_NAME_SIZE],
    lines: u32,
    fd: i32,
}

#[repr(C)]
struct GpiohandleData {
    values: [u8; GPIOHANDLES_MAX],
}

#[doc = "GPIO pin using Linux character device interface"]
pub struct ChardevPin {
    chip_path: String,
    line_fd: RawFd,
    pin: u8,
    direction: Direction,
}

impl ChardevPin {
    #[doc = "Create a new ChardevPin instance"]
    pub fn new(chip: &str, pin: u8, direction: Direction) -> Result<Self> {
        let line_fd = Self::request_line(chip, pin, direction)?;
        Ok(ChardevPin {
            chip_path: chip.to_string(),
            line_fd,
            pin,
            direction,
        })
    }

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
    fn set_high(&mut self) -> Result<()> {
        self.write_value(1)
    }

    fn set_low(&mut self) -> Result<()> {
        self.write_value(0)
    }

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
    fn drop(&mut self) {
        sys_utils::close(self.line_fd);
    }
}
