//! Ferropin - A Rust library for GPIO, I2C, and SSD1306 display interaction on Linux
//!
//! This crate provides hardware abstractions for interacting with GPIO pins, I2C devices,
//! and SSD1306 OLED displays on Linux systems.
//!
//! # Features
//!
//! * GPIO access via Linux character device interface
//! * I2C communication (hardware and bit-banged implementations)
//! * SSD1306 OLED display driver
//! * Error handling with location tracking
//! * Low-level system utilities for ARM64
//!
//! # Usage
//!
//! Add to your `Cargo.toml`:
//!
//! ```toml
//! [dependencies]
//! ferropin = { path = "path/to/ferropin" }
//! ```
//!
//! # Examples
//!
//! See the `sandbox` binary in `src/bin/sandbox.rs` for usage examples.

pub mod display;
pub mod error;
pub mod gpio;
pub mod i2c;
pub mod sys_utils;
