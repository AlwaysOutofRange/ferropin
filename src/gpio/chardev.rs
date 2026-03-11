use std::{fs::OpenOptions, os::fd::{AsRawFd, RawFd}};

use crate::{err, error::{ErrorKind, Result}, gpio::{Direction, GpioPin}, try_io};

const GPIO_MAX_NAME_SIZE: usize = 32;
const GPIOHANDLES_MAX:    usize = 64;
const GPIOHANDLE_REQUEST_INPUT:  u32 = 1 << 0;
const GPIOHANDLE_REQUEST_OUTPUT: u32 = 1 << 1;
const GPIO_GET_LINEHANDLE_IOCTL:          u64 = 0xC16CB403;
const GPIOHANDLE_GET_LINE_VALUES_IOCTL:   u64 = 0xC040B408;
const GPIOHANDLE_SET_LINE_VALUES_IOCTL:   u64 = 0xC040B409;

#[repr(C)]
struct GpiohandleRequest {
    lineoffsets: [u32; GPIOHANDLES_MAX],
    flags: u32,
    default_values: [u8; GPIOHANDLES_MAX],
    consumer_label: [u8; GPIO_MAX_NAME_SIZE],
    lines: u32,
    fd: i32
}

#[repr(C)]
struct GpiohandleData {
    values: [u8; GPIOHANDLES_MAX]
}

#[allow(dead_code)]
pub struct ChardevPin {
    _chip_file: std::fs::File, // keep chip file alive
    line_fd: RawFd,
    pin: u8
}

impl ChardevPin {
    pub fn new(chip: &str, pin: u8, direction: Direction) -> Result<Self> {
        // Step 1. Open chip file (e.g /dev/gpiochip0)
        let chip_file = try_io!(OpenOptions::new()
            .read(true)
            .write(true)
            .open(chip));

        let chip_fd = chip_file.as_raw_fd();

        // Step 2. build the line request
        let mut label = [0u8; GPIO_MAX_NAME_SIZE];
        let name = b"ferropin";
        label[..name.len()].copy_from_slice(name);

        let flags = match direction {
            Direction::Input => GPIOHANDLE_REQUEST_INPUT,
            Direction::Output => GPIOHANDLE_REQUEST_OUTPUT
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

        // Step 3. ask the kernel for the handle of the line
        let ret = sys_utils::ioctl(chip_fd, GPIO_GET_LINEHANDLE_IOCTL, &mut req as *mut GpiohandleRequest as *mut u8);
        if ret < 0 || req.fd < 0 {
            return Err(err!(ErrorKind::InvalidPin(pin)));
        }

        Ok(ChardevPin {
            _chip_file: chip_file,
            line_fd: req.fd,
            pin
        })
    }

    fn write_value(&self, value: u8) -> Result<()> {
        let mut data = GpiohandleData {
            values: [0u8; GPIOHANDLES_MAX]
        };
        data.values[0] = value;

        let ret = sys_utils::ioctl(self.line_fd, GPIOHANDLE_SET_LINE_VALUES_IOCTL, &mut data as *mut GpiohandleData as *mut u8);
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
            values: [0u8; GPIOHANDLES_MAX]
        };

        let ret = sys_utils::ioctl(self.line_fd, GPIOHANDLE_GET_LINE_VALUES_IOCTL, &mut data as *mut GpiohandleData as *mut u8);
        if ret < 0 {
            return Err(err!(ErrorKind::Io(std::io::Error::last_os_error())));
        }

        Ok(data.values[0] != 0)
    }
}

impl Drop for ChardevPin {
    fn drop(&mut self) {
        sys_utils::close(self.line_fd);
    }
}


mod sys_utils {
    use std::os::fd::RawFd;

    const SYS_IOCTL: i64 = 29;
    const SYS_CLOSE: i64 = 57;

    pub fn ioctl(fd: RawFd, request: u64, arg: *mut u8) -> i64 {
        unsafe {
            let ret: i64;

            std::arch::asm!(
                "svc #0",
                in("x8") SYS_IOCTL,
                in("x0") fd as i64,
                in("x1") request,
                in("x2") arg,
                lateout("x0") ret,
                options(nostack)
            );
            ret
        }
    }

    pub fn close(fd: RawFd) -> i64 {
        unsafe {
            let ret: i64;

            std::arch::asm!(
                "svc #0",
                in("x8") SYS_CLOSE,
                in("x0") fd as i64,
                lateout("x0") ret,
                options(nostack)
            );
            ret
        }
    }
}
