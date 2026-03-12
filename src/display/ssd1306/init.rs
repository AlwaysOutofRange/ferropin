use crate::{
    display::ssd1306::{cmd::*, Ssd1306},
    error::Result,
    i2c::I2c,
};

impl<B: I2c> Ssd1306<B> {
    // Init sequence for 128x64 SSD1306. Called by Ssd1306::new().
    pub(super) fn init(&mut self) -> Result<()> {
        self.cmd(CMD_DISPLAY_OFF, &[])?;

        self.cmd(CMD_SET_DISPLAY_CLOCK, &[0x80])?;
        self.cmd(CMD_SET_MUX_RATIO, &[0x3F])?; // 64 rows
        self.cmd(CMD_SET_DISPLAY_OFFSET, &[0x00])?;
        self.cmd(CMD_SET_START_LINE | 0, &[])?;

        self.cmd(CMD_CHARGE_PUMP, &[0x14])?; // enable
        self.cmd(CMD_SET_MEMORY_MODE, &[0x00])?; // horizontal addressing

        self.cmd(CMD_SET_SEGMENT_REMAP, &[])?;
        self.cmd(CMD_SET_COM_SCAN_DIR, &[])?;

        self.cmd(CMD_SET_COM_PINS, &[0x12])?;
        self.cmd(CMD_SET_CONTRAST, &[0xCF])?;
        self.cmd(CMD_SET_PRECHARGE, &[0xF1])?;
        self.cmd(CMD_SET_VCOM_DESELECT, &[0x40])?;

        self.cmd(CMD_ENTIRE_DISPLAY_ON, &[])?;
        self.cmd(CMD_SET_NORMAL_DISPLAY, &[])?;

        self.framebuffer.clear();
        self.flush()?;

        self.cmd(CMD_DISPLAY_ON, &[])?;

        Ok(())
    }
}
