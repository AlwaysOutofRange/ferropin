//! Ferropin
//!
//! A Rust library for interacting with GPIO, I2C, and display devices (specifically SSD1306 OLED displays) on Linux systems.
//!
//! # Features
//!
//! * GPIO access via character device interface
//! * I2C communication (both hardware and bit-banged)
//! * SSD1306 OLED display driver
//!
//! # Usage
//!
//! Add this to your `Cargo.toml`:
//!
//! ```toml
//! [dependencies]
//! ferropin = { path = "path/to/ferropin" }
//! ```
//!
//! Then, in your crate root:
//!
//! ```rust
//! extern crate ferropin;
//! ```
//!
//! # Examples
//!
//! See the `sandbox` binary in `src/bin/sandbox.rs` for usage examples.
//!
pub mod display;
pub mod error;
pub mod gpio;
pub mod i2c;
pub mod sys_utils;
