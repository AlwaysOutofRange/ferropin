# Ferropin

A Rust library for interacting with GPIO, I2C, and display devices (specifically SSD1306 OLED displays) on Linux systems.

## Disclaimer

This project is primarily developed for personal use. Contributions and pull requests for additional features, improvements, or compatibility with other hardware are welcome.

## Project Structure

```
src/
├── bin/
│   └── sandbox.rs        # Example binary demonstrating usage
├── display/
│   ├── mod.rs            # Display module re-exports
│   └── ssd1306/          # SSD1306 OLED display driver
│       ├── cmd.rs        # SSD1306 command definitions
│       ├── framebuffer.rs# Framebuffer implementation
│       ├── init.rs       # Display initialization sequences
│       └── mod.rs        # SSD1306 driver implementation
├── gpio/
│   ├── mod.rs            # GPIO module re-exports
│   └── chardev.rs        # GPIO character device interface
├── i2c/
│   ├── mod.rs            # I2C module re-exports
│   ├── bitbang.rs        # Bit-banged I2C implementation
│   └── hardware.rs       # Hardware I2C implementation
├── error.rs              # Error definitions
├── lib.rs                # Library root
└── sys_utils.rs          # System utility functions
```

## Modules

### Display (`display::ssd1306`)
Interface for SSD1306-based OLED displays via I2C.

Features:
- Drawing primitives (set_pixel)
- Framebuffer management
- Display initialization
- Hardware I2C and bit-banged I2C support

### GPIO (`gpio::chardev`)
Interface for controlling GPIO pins via the Linux character device interface (`/dev/gpiochip*`).

Features:
- Pin direction configuration
- Pin value reading/writing

### I2C (`i2c`)
Abstractions for I2C communication.

Implementations:
- `hardware::HardwareI2c`: Uses Linux I2C device files
- `bitbang::BitbangI2c`: Software I2C using GPIO pins

## Usage

Add to your `Cargo.toml`:

```toml
[dependencies]
ferropin = { path = "path/to/ferropin" }
```

## Building

```bash
cargo build --release
```

## Running Examples

The project includes a sandbox binary that demonstrates various features:

```bash
cargo run --bin sandbox
```

Note: The sandbox example requires appropriate hardware connections and permissions to access GPIO and I2C devices.

## License

This project is licensed under the MIT License - see the LICENSE file for details.