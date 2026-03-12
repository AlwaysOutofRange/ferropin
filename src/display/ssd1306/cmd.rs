// SSD1306 command constants
// See datasheet: https://cdn-shop.adafruit.com/datasheets/SSD1306.pdf

pub const CTRL_CMD: u8 = 0x00;
pub const CTRL_DATA: u8 = 0x40;

pub const CMD_DISPLAY_OFF: u8 = 0xAE;
pub const CMD_DISPLAY_ON: u8 = 0xAF;
pub const CMD_SET_CONTRAST: u8 = 0x81;
pub const CMD_ENTIRE_DISPLAY_ON: u8 = 0xA4;
pub const CMD_SET_NORMAL_DISPLAY: u8 = 0xA6;
pub const CMD_SET_MUX_RATIO: u8 = 0xA8;
pub const CMD_SET_DISPLAY_OFFSET: u8 = 0xD3;
pub const CMD_SET_DISPLAY_CLOCK: u8 = 0xD5;
pub const CMD_SET_PRECHARGE: u8 = 0xD9;
pub const CMD_SET_COM_PINS: u8 = 0xDA;
pub const CMD_SET_VCOM_DESELECT: u8 = 0xDB;
pub const CMD_SET_START_LINE: u8 = 0x40;
pub const CMD_SET_SEGMENT_REMAP: u8 = 0xA1;
pub const CMD_SET_COM_SCAN_DIR: u8 = 0xC8;
pub const CMD_SET_MEMORY_MODE: u8 = 0x20;
pub const CMD_SET_COL_ADDR: u8 = 0x21;
pub const CMD_SET_PAGE_ADDR: u8 = 0x22;
pub const CMD_CHARGE_PUMP: u8 = 0x8D;
