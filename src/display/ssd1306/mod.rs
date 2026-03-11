//! SSD1306 OLED display driver for the ferropin crate.
//!
//! This module provides a driver for SSD1306-based OLED displays communicating
//! via I2C.

use crate::{
    display::ssd1306::{
        cmd::*,
        framebuffer::{FrameBuffer, PAGES, WIDTH},
    },
    error::Result,
    i2c::I2c,
};

/// SSD1306 command definitions
pub mod cmd;
/// Framebuffer implementation for pixel storage
pub mod framebuffer;
/// Display initialization sequences
pub mod init;

/// Default I2C address for SSD1306 displays
const SSD1306_ADDR: u8 = 0x3C;

/// SSD1306 OLED display driver
///
/// This struct provides an interface for controlling SSD1306-based OLED displays
/// via I2C communication.
pub struct Ssd1306<B: I2c> {
    /// I2C bus used for communication
    bus: B,
    /// Framebuffer for storing pixel data
    framebuffer: FrameBuffer,
}

impl<B: I2c> Ssd1306<B> {
    /// Create a new SSD1306 display instance with initialization
    ///
    /// # Arguments
    ///
    /// * `bus` - An I2C bus implementation
    ///
    /// # Returns
    ///
    /// A Result containing the new Ssd1306 instance or an error
    pub fn new(bus: B) -> Result<Self> {
        let mut display = Ssd1306 {
            bus,
            framebuffer: FrameBuffer::new(),
        };
        display.init()?;

        Ok(display)
    }

    /// Create a new SSD1306 display instance without initialization
    ///
    /// This is useful when you want to configure the display manually.
    ///
    /// # Arguments
    ///
    /// * `bus` - An I2C bus implementation
    ///
    /// # Returns
    ///
    /// An uninitialized Ssd1306 instance
    pub fn new_uninit(bus: B) -> Self {
        Ssd1306 {
            bus,
            framebuffer: FrameBuffer::new(),
        }
    }

    /// Create a new SSD1306 display instance with memory mode configuration
    ///
    /// This sets up the display with horizontal addressing mode.
    ///
    /// # Arguments
    ///
    /// * `bus` - An I2C bus implementation
    ///
    /// # Returns
    ///
    /// A Result containing the new Ssd1306 instance or an error
    pub fn new_takeover(bus: B) -> Result<Self> {
        let mut display = Ssd1306 {
            bus,
            framebuffer: FrameBuffer::new(),
        };

        display.cmd(CMD_SET_MEMORY_MODE, &[0x00])?;
        display.cmd(CMD_SET_COL_ADDR, &[0, (WIDTH - 1) as u8])?;
        display.cmd(CMD_SET_PAGE_ADDR, &[0, (PAGES - 1) as u8])?;

        Ok(display)
    }

    /// Send a command to the display
    ///
    /// # Arguments
    ///
    /// * `command` - The command byte to send
    /// * `args` - Optional arguments for the command
    ///
    /// # Returns
    ///
    /// A Result indicating success or failure
    pub(super) fn cmd(&mut self, command: u8, args: &[u8]) -> Result<()> {
        let mut buf = [0u8; 16];
        buf[0] = CTRL_CMD;
        buf[1] = command;
        buf[2..2 + args.len()].copy_from_slice(args);
        self.bus.write(SSD1306_ADDR, &buf[..2 + args.len()])
    }

    /// Clear the display
    ///
    /// This clears the framebuffer but doesn't update the display.
    /// Call `flush()` to update the display.
    pub fn clear(&mut self) {
        self.framebuffer.clear();
    }

    /// Fill the entire display
    ///
    /// This fills the framebuffer with all pixels on.
    /// Call `flush()` to update the display.
    pub fn fill(&mut self) {
        self.framebuffer.fill();
    }

    /// Set a pixel at the specified coordinates
    ///
    /// # Arguments
    ///
    /// * `x` - X coordinate (0-127)
    /// * `y` - Y coordinate (0-63)
    /// * `on` - true to turn the pixel on, false to turn it off
    pub fn set_pixel(&mut self, x: usize, y: usize, on: bool) {
        self.framebuffer.set_pixel(x, y, on);
    }

    /// Get the value of a pixel at the specified coordinates
    ///
    /// # Arguments
    ///
    /// * `x` - X coordinate (0-127)
    /// * `y` - Y coordinate (0-63)
    ///
    /// # Returns
    ///
    /// true if the pixel is on, false if off
    pub fn get_pixel(&self, x: usize, y: usize) -> bool {
        self.framebuffer.get_pixel(x, y)
    }

    /// Update the display with the contents of the framebuffer
    ///
    /// This sends the entire framebuffer to the display.
    ///
    /// # Returns
    ///
    /// A Result indicating success or failure
    pub fn flush(&mut self) -> Result<()> {
        // Set the full display area
        self.cmd(CMD_SET_COL_ADDR, &[0, (WIDTH - 1) as u8])?;
        self.cmd(CMD_SET_PAGE_ADDR, &[0, (PAGES - 1) as u8])?;

        // Prepare the data buffer (control byte + framebuffer data)
        let mut buf = [0u8; 1 + WIDTH * PAGES];
        buf[0] = CTRL_DATA;
        buf[1..].copy_from_slice(&self.framebuffer.buf);
        self.bus.write(SSD1306_ADDR, &buf)
    }

    /// Turn the display on or off
    ///
    /// # Arguments
    ///
    /// * `on` - true to turn the display on, false to turn it off
    ///
    /// # Returns
    ///
    /// A Result indicating success or failure
    pub fn set_display_on(&mut self, on: bool) -> Result<()> {
        self.cmd(if on { CMD_DISPLAY_ON } else { CMD_DISPLAY_OFF }, &[])
    }

    /// Set the display contrast
    ///
    /// # Arguments
    ///
    /// * `contrast` - Contrast value (0-255)
    ///
    /// # Returns
    ///
    /// A Result indicating success or failure
    pub fn set_contrast(&mut self, contrast: u8) -> Result<()> {
        self.cmd(CMD_SET_CONTRAST, &[contrast])
    }
}
