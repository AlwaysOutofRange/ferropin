use std::os::fd::RawFd;

use crate::{
    err,
    error::{ErrorKind, Result},
    i2c::I2c,
    sys_utils,
};

/// Hardware I2C via `/dev/i2c-*`.
pub struct HardwareI2c {
    fd: RawFd,
    current_addr: u8,
}

impl HardwareI2c {
    pub fn new(bus: u8) -> Result<Self> {
        let path = format!("/dev/i2c-{}", bus);

        let fd = sys_utils::open(path.as_ptr(), sys_utils::O_RDWR);
        if fd < 0 {
            return Err(err!(ErrorKind::Io(std::io::Error::last_os_error())));
        }

        Ok(HardwareI2c {
            fd: fd as RawFd,
            current_addr: 0xFF,
        })
    }

    fn set_addr(&mut self, addr: u8) -> Result<()> {
        if self.current_addr == addr {
            return Ok(());
        }

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

        let ret = sys_utils::write(self.fd, data.as_ptr(), data.len());
        if ret < 0 {
            return Err(err!(ErrorKind::Io(std::io::Error::last_os_error())));
        }

        Ok(())
    }

    fn read(&mut self, addr: u8, buf: &mut [u8]) -> Result<()> {
        self.set_addr(addr)?;

        let ret = sys_utils::read(self.fd, buf.as_mut_ptr(), buf.len());
        if ret < 0 {
            return Err(err!(ErrorKind::Io(std::io::Error::last_os_error())));
        }

        Ok(())
    }

    fn write_read(&mut self, addr: u8, write: &[u8], read: &mut [u8]) -> Result<()> {
        self.write(addr, write)?;
        self.read(addr, read)?;

        Ok(())
    }
}

impl Drop for HardwareI2c {
    fn drop(&mut self) {
        sys_utils::close(self.fd);
    }
}
