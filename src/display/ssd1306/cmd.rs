//! SSD1306 command definitions for the ferropin crate.
//!
//! This module contains constants for SSD1306 OLED display commands.
//! These are used internally by the SSD1306 driver to control the display.
//!
//! # Control Bytes
//!
//! * `CTRL_CMD`: Control byte for command data
//! * `CTRL_DATA`: Control byte for display data
//!
//! # Commands
//!
//! The SSD1306 display module supports various commands for:
//! * Display on/off control
//! * Contrast setting
//! * Memory addressing
//! * Display configuration
//!
//! For detailed information about each command, refer to the SSD1306 datasheet.
//!
pub const CTRL_CMD: u8 = 0x00;
pub const CTRL_DATA: u8 = 0x40;

/// Display off command
pub const CMD_DISPLAY_OFF: u8 = 0xAE;
/// Display on command
pub const CMD_DISPLAY_ON: u8 = 0xAF;
/// Set contrast command
pub const CMD_SET_CONTRAST: u8 = 0x81;
/// Entire display on (all pixels on)
pub const CMD_ENTIRE_DISPLAY_ON: u8 = 0xA4;
/// Set normal display (not inverted)
pub const CMD_SET_NORMAL_DISPLAY: u8 = 0xA6;
/// Set mux ratio (for display height)
pub const CMD_SET_MUX_RATIO: u8 = 0xA8;
/// Set display offset
pub const CMD_SET_DISPLAY_OFFSET: u8 = 0xD3;
/// Set display clock divide ratio/oscillator frequency
pub const CMD_SET_DISPLAY_CLOCK: u8 = 0xD5;
/// Set pre-charge period
pub const CMD_SET_PRECHARGE: u8 = 0xD9;
/// Set COM pins hardware configuration
pub const CMD_SET_COM_PINS: u8 = 0xDA;
/// Set V_COMH deselect level
pub const CMD_SET_VCOM_DESELECT: u8 = 0xDB;
/// Set display start line
pub const CMD_SET_START_LINE: u8 = 0x40;
/// Set segment remap (column address mapping)
pub const CMD_SET_SEGMENT_REMAP: u8 = 0xA1;
/// Set COM output scan direction
pub const CMD_SET_COM_SCAN_DIR: u8 = 0xC8;
/// Set memory addressing mode
pub const CMD_SET_MEMORY_MODE: u8 = 0x20;
/// Set column address
pub const CMD_SET_COL_ADDR: u8 = 0x21;
/// Set page address
pub const CMD_SET_PAGE_ADDR: u8 = 0x22;
/// Charge pump setting
pub const CMD_CHARGE_PUMP: u8 = 0x8D;
