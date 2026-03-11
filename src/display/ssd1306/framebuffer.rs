//! Framebuffer for the SSD1306 OLED display driver for the ferropin crate.
//!
//! This module provides a framebuffer implementation for storing and manipulating
//! pixel data for 128x64 SSD1306 OLED displays.

/// Display width in pixels
pub const WIDTH: usize = 128;
/// Display height in pixels
pub const HEIGHT: usize = 64;
/// Number of pages (each page represents 8 vertical pixels)
pub const PAGES: usize = HEIGHT / 8;

/// Framebuffer for the SSD1306 OLED display
///
/// The framebuffer stores pixel data in a format compatible with the SSD1306 display.
/// It uses a page-based memory layout where each byte represents 8 vertical pixels.
pub struct FrameBuffer {
    /// Raw buffer data (128 columns × 8 pages)
    pub buf: [u8; WIDTH * PAGES],
}

impl FrameBuffer {
    /// Create a new empty framebuffer
    ///
    /// # Returns
    ///
    /// A new FrameBuffer instance with all pixels cleared (off)
    pub fn new() -> Self {
        FrameBuffer {
            buf: [0u8; WIDTH * PAGES],
        }
    }

    /// Clear the entire framebuffer (all pixels off)
    pub fn clear(&mut self) {
        self.buf.fill(0);
    }

    /// Fill the entire framebuffer (all pixels on)
    pub fn fill(&mut self) {
        self.buf.fill(0xFF);
    }

    /// Set a pixel at the specified coordinates
    ///
    /// # Arguments
    ///
    /// * `x` - X coordinate (0-127)
    /// * `y` - Y coordinate (0-63)
    /// * `on` - true to turn the pixel on, false to turn it off
    pub fn set_pixel(&mut self, x: usize, y: usize, on: bool) {
        if x >= WIDTH || y >= HEIGHT {
            return;
        }

        // Calculate the byte index and bit position
        let index = (y / 8) * WIDTH + x;
        let bit = y % 8;

        if on {
            self.buf[index] |= 1 << bit;
        } else {
            self.buf[index] &= !(1 << bit);
        }
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
        if x >= WIDTH || y >= HEIGHT {
            return false;
        }

        // Calculate the byte index and bit position
        let index = (y / 8) * WIDTH + x;
        let bit = y % 8;

        self.buf[index] & (1 << bit) != 0
    }
}
