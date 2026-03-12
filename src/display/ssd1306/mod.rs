use crate::{
    display::{
        framebuffer::FrameBuffer,
        ssd1306::{
            cmd::*,
            fonts::{get_char_columns, TextStyle, CHAR_H, CHAR_W},
        },
    },
    error::Result,
    i2c::I2c,
};

pub mod cmd;
pub mod init;

pub mod fonts;

const SSD1306_ADDR: u8 = 0x3C;

const WIDTH: usize = 128;
const HEIGHT: usize = 64;
const PAGES: usize = HEIGHT / 8;
const BUF_SIZE: usize = WIDTH * PAGES;

/// SSD1306 OLED display driver over I2C.
pub struct Ssd1306<B: I2c> {
    bus: B,
    framebuffer: FrameBuffer<WIDTH, HEIGHT, BUF_SIZE>,
}

impl<B: I2c> Ssd1306<B> {
    pub fn new(bus: B) -> Result<Self> {
        let mut display = Ssd1306 {
            bus,
            framebuffer: FrameBuffer::new(),
        };
        display.init()?;

        Ok(display)
    }

    pub(super) fn cmd(&mut self, command: u8, args: &[u8]) -> Result<()> {
        let mut buf = [0u8; 16];
        buf[0] = CTRL_CMD;
        buf[1] = command;
        buf[2..2 + args.len()].copy_from_slice(args);
        self.bus.write(SSD1306_ADDR, &buf[..2 + args.len()])
    }

    pub fn clear(&mut self) {
        self.framebuffer.clear();
    }

    pub fn fill(&mut self) {
        self.framebuffer.fill();
    }

    pub fn set_pixel(&mut self, x: usize, y: usize, on: bool) {
        self.framebuffer.set_pixel(x, y, on);
    }

    pub fn get_pixel(&self, x: usize, y: usize) -> bool {
        self.framebuffer.get_pixel(x, y)
    }

    /// Send the framebuffer to the display.
    pub fn flush(&mut self) -> Result<()> {
        self.cmd(CMD_SET_COL_ADDR, &[0, (WIDTH - 1) as u8])?;
        self.cmd(CMD_SET_PAGE_ADDR, &[0, (PAGES - 1) as u8])?;

        let mut buf = [0u8; 1 + BUF_SIZE];
        buf[0] = CTRL_DATA;
        buf[1..].copy_from_slice(&self.framebuffer.buf);
        self.bus.write(SSD1306_ADDR, &buf)
    }

    pub fn set_display_on(&mut self, on: bool) -> Result<()> {
        self.cmd(if on { CMD_DISPLAY_ON } else { CMD_DISPLAY_OFF }, &[])
    }

    pub fn set_contrast(&mut self, contrast: u8) -> Result<()> {
        self.cmd(CMD_SET_CONTRAST, &[contrast])
    }

    pub fn draw_char(&mut self, x: usize, y: usize, c: char, style: TextStyle) {
        let cols = get_char_columns(c, style);
        let scale = style.scale as usize;

        for col in 0..CHAR_W {
            let byte = cols[col];
            for row in 0..CHAR_H {
                let on = (byte >> row) & 1 == 1;
                for sx in 0..scale {
                    for sy in 0..scale {
                        let px = x + col * scale + sx;
                        let py = y + row * scale + sy;

                        self.framebuffer.set_pixel(px, py, on);
                    }
                }
            }
        }
    }

    pub fn draw_text(&mut self, x: usize, y: usize, text: &str, style: TextStyle) {
        let char_w = CHAR_W * style.scale as usize;
        let mut cx = x;

        for c in text.chars() {
            if cx + char_w > WIDTH {
                break;
            }

            self.draw_char(cx, y, c, style);
            cx += char_w;
        }
    }
}
