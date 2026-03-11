use crate::{
    display::ssd1306::{
        cmd::*,
        framebuffer::{FrameBuffer, PAGES, WIDTH},
    },
    error::Result,
    i2c::I2c,
};

pub mod cmd;
pub mod framebuffer;
pub mod init;

const SSD1306_ADDR: u8 = 0x3C;

#[doc = "SSD1306 OLED display driver"]
pub struct Ssd1306<B: I2c> {
    bus: B,
    framebuffer: FrameBuffer,
}

impl<B: I2c> Ssd1306<B> {
    #[doc = "Create a new SSD1306 display instance with initialization"]
    pub fn new(bus: B) -> Result<Self> {
        let mut display = Ssd1306 {
            bus,
            framebuffer: FrameBuffer::new(),
        };
        display.init()?;

        Ok(display)
    }

    #[doc = "Create a new SSD1306 display instance without initialization"]
    pub fn new_uninit(bus: B) -> Self {
        Ssd1306 {
            bus,
            framebuffer: FrameBuffer::new(),
        }
    }

    #[doc = "Create a new SSD1306 display instance with memory mode configuration"]
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

    pub(super) fn cmd(&mut self, command: u8, args: &[u8]) -> Result<()> {
        let mut buf = [0u8; 16];
        buf[0] = CTRL_CMD;
        buf[1] = command;
        buf[2..2 + args.len()].copy_from_slice(args);
        self.bus.write(SSD1306_ADDR, &buf[..2 + args.len()])
    }

    #[doc = "Clear the display"]
    pub fn clear(&mut self) {
        self.framebuffer.clear();
    }

    #[doc = "Fill the entire display"]
    pub fn fill(&mut self) {
        self.framebuffer.fill();
    }

    #[doc = "Set a pixel at the specified coordinates (x: 0-127, y: 0-63)"]
    pub fn set_pixel(&mut self, x: usize, y: usize, on: bool) {
        self.framebuffer.set_pixel(x, y, on);
    }

    #[doc = "Get the value of a pixel at the specified coordinates"]
    pub fn get_pixel(&self, x: usize, y: usize) -> bool {
        self.framebuffer.get_pixel(x, y)
    }

    #[doc = "Update the display with the contents of the framebuffer"]
    pub fn flush(&mut self) -> Result<()> {
        self.cmd(CMD_SET_COL_ADDR, &[0, (WIDTH - 1) as u8])?;
        self.cmd(CMD_SET_PAGE_ADDR, &[0, (PAGES - 1) as u8])?;

        let mut buf = [0u8; 1 + WIDTH * PAGES];
        buf[0] = CTRL_DATA;
        buf[1..].copy_from_slice(&self.framebuffer.buf);
        self.bus.write(SSD1306_ADDR, &buf)
    }

    #[doc = "Turn the display on or off"]
    pub fn set_display_on(&mut self, on: bool) -> Result<()> {
        self.cmd(if on { CMD_DISPLAY_ON } else { CMD_DISPLAY_OFF }, &[])
    }

    #[doc = "Set the display contrast (0-255)"]
    pub fn set_contrast(&mut self, contrast: u8) -> Result<()> {
        self.cmd(CMD_SET_CONTRAST, &[contrast])
    }
}
