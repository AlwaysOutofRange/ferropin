use crate::error::Result;

pub mod chardev;

#[doc = "Direction a GPIO pin can be configured for"]
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Direction {
    #[doc = "Pin configured as input"]
    Input,
    #[doc = "Pin configured as output"]
    Output,
}

#[doc = "Trait defining the interface for GPIO pins"]
pub trait GpioPin {
    #[doc = "Set the pin to a high voltage level"]
    fn set_high(&mut self) -> Result<()>;

    #[doc = "Set the pin to a low voltage level (ground)"]
    fn set_low(&mut self) -> Result<()>;

    #[doc = "Read the current value of the pin (returns true if high)"]
    fn read(&self) -> Result<bool>;

    #[doc = "Set the direction of the pin (input or output)"]
    fn set_direction(&mut self, direction: Direction) -> Result<()>;

    #[doc = "Set the pin to high or low based on a boolean value"]
    fn set(&mut self, high: bool) -> Result<()> {
        if high {
            self.set_high()
        } else {
            self.set_low()
        }
    }
}
