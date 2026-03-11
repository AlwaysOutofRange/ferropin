use crate::error::Result;

pub mod bitbang;
pub mod hardware;

#[doc = "Trait defining the interface for I2C communication"]
pub trait I2c {
    #[doc = "Write data to an I2C device"]
    fn write(&mut self, addr: u8, data: &[u8]) -> Result<()>;

    #[doc = "Read data from an I2C device"]
    fn read(&mut self, addr: u8, buf: &mut [u8]) -> Result<()>;

    #[doc = "Write data to then read data from an I2C device"]
    fn write_read(&mut self, addr: u8, write: &[u8], read: &mut [u8]) -> Result<()>;
}
