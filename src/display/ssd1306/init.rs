//! SSD1306 display initialization for the ferropin crate.
//!
//! This module handles the initialization sequence for SSD1306 OLED displays.
//!
//! The initialization process configures the display with common settings for
//! a 128x64 SSD1306 OLED display, including:
//!
//! * Display off during configuration
//! * Clock and mux ratio settings
//! * Memory addressing mode
//! * Segment and COM pin configuration
//! * Contrast and pre-charge settings
//! * Charge pump enablement
//!
//! Note: This is an internal module used by the SSD1306 driver.
//!
use crate::{
    display::ssd1306::{cmd::*, Ssd1306},
    error::Result,
    i2c::I2c,
};

impl<B: I2c> Ssd1306<B> {
    /// Initialize the SSD1306 display with default settings
    ///
    /// This method configures the display with standard settings for
    /// a 128x64 SSD1306 OLED display. It's called automatically by `Ssd1306::new()`.
    ///
    /// # Returns
    ///
    /// A Result indicating success or failure
    pub(super) fn init(&mut self) -> Result<()> {
        // Turn display off during configuration
        self.cmd(CMD_DISPLAY_OFF, &[])?;

        // Set display clock divide ratio/oscillator frequency
        self.cmd(CMD_SET_DISPLAY_CLOCK, &[0x80])?;

        // Set mux ratio (for 64 rows: 0x3F = 63, which corresponds to 64 rows)
        self.cmd(CMD_SET_MUX_RATIO, &[0x3F])?;

        // Set display offset (no offset)
        self.cmd(CMD_SET_DISPLAY_OFFSET, &[0x00])?;

        // Set display start line (line 0)
        self.cmd(CMD_SET_START_LINE | 0, &[])?;

        // Enable charge pump
        self.cmd(CMD_CHARGE_PUMP, &[0x14])?;

        // Set memory addressing mode to horizontal
        self.cmd(CMD_SET_MEMORY_MODE, &[0x00])?;

        // Set segment remap (column address 0 mapped to SEG0)
        self.cmd(CMD_SET_SEGMENT_REMAP, &[])?;

        // Set COM output scan direction (remapped mode)
        self.cmd(CMD_SET_COM_SCAN_DIR, &[])?;

        // Set COM pins hardware configuration
        self.cmd(CMD_SET_COM_PINS, &[0x12])?;

        // Set contrast
        self.cmd(CMD_SET_CONTRAST, &[0xCF])?;

        // Set pre-charge period
        self.cmd(CMD_SET_PRECHARGE, &[0xF1])?;

        // Set V_COMH deselect level
        self.cmd(CMD_SET_VCOM_DESELECT, &[0x40])?;

        // Entire display on (not GDDRAM content)
        self.cmd(CMD_ENTIRE_DISPLAY_ON, &[])?;

        // Set normal display (not inverted)
        self.cmd(CMD_SET_NORMAL_DISPLAY, &[])?;

        // Clear the framebuffer and flush to display
        self.framebuffer.clear();
        self.flush()?;

        // Turn display on
        self.cmd(CMD_DISPLAY_ON, &[])?;

        Ok(())
    }
}
