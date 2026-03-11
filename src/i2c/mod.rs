use crate::error::Result;

pub mod bitbang;
pub mod hardware;

pub trait I2c {
    fn write(&mut self, addr: u8, data: &[u8]) -> Result<()>;
    fn read(&mut self, addr: u8, buf: &mut [u8]) -> Result<()>;
    fn write_read(&mut self, addr: u8, write: &[u8], read: &mut [u8]) -> Result<()>;
}
