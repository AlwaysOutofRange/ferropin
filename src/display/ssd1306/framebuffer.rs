#[doc = "Display width in pixels"]
pub const WIDTH: usize = 128;
#[doc = "Display height in pixels"]
pub const HEIGHT: usize = 64;
#[doc = "Number of pages (HEIGHT / 8)"]
pub const PAGES: usize = HEIGHT / 8;

#[doc = "Framebuffer for the SSD1306 OLED display"]
pub struct FrameBuffer {
    pub buf: [u8; WIDTH * PAGES],
}

impl FrameBuffer {
    #[doc = "Create a new empty framebuffer"]
    pub fn new() -> Self {
        FrameBuffer {
            buf: [0u8; WIDTH * PAGES],
        }
    }

    #[doc = "Clear all pixels"]
    pub fn clear(&mut self) {
        self.buf.fill(0);
    }

    #[doc = "Fill all pixels"]
    pub fn fill(&mut self) {
        self.buf.fill(0xFF);
    }

    #[doc = "Set a pixel at (x, y)"]
    pub fn set_pixel(&mut self, x: usize, y: usize, on: bool) {
        if x >= WIDTH || y >= HEIGHT {
            return;
        }

        let index = (y / 8) * WIDTH + x;
        let bit = y % 8;

        if on {
            self.buf[index] |= 1 << bit;
        } else {
            self.buf[index] &= !(1 << bit);
        }
    }

    #[doc = "Get pixel value at (x, y)"]
    pub fn get_pixel(&self, x: usize, y: usize) -> bool {
        if x >= WIDTH || y >= HEIGHT {
            return false;
        }

        let index = (y / 8) * WIDTH + x;
        let bit = y % 8;

        self.buf[index] & (1 << bit) != 0
    }
}
