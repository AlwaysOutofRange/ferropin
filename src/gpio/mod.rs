use crate::error::Result;

pub mod chardev;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Direction {
    Input,
    Output,
}

pub trait GpioPin {
    fn set_high(&mut self) -> Result<()>;
    fn set_low(&mut self) -> Result<()>;
    fn read(&self) -> Result<bool>;
    fn set_direction(&mut self, direction: Direction) -> Result<()>;
    fn set(&mut self, high: bool) -> Result<()> {
        if high {
            self.set_high()
        } else {
            self.set_low()
        }
    }
}
