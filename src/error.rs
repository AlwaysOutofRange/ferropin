use std::fmt;

/// Represents the location where an error occurred
#[derive(Debug)]
pub struct Location {
    /// The source file where the error occurred
    pub file: &'static str,
    /// The line number where the error occurred
    pub line: u32,
}

/// Main error type for the ferropin library
///
/// Contains both the error kind and location information for debugging.
#[derive(Debug)]
pub struct Error {
    /// The specific type of error that occurred
    pub kind: ErrorKind,
    /// Location information for where the error occurred
    pub location: Location,
}

/// Different categories of errors that can occur
#[derive(Debug)]
pub enum ErrorKind {
    /// An I/O error occurred (e.g., failure to read/write to a device file)
    Io(std::io::Error),
    /// An invalid pin number was specified
    InvalidPin(u8),
    /// An I2C device did not acknowledge a transmission
    I2cNack,
    /// An I2C operation timed out
    I2cTimeout,
    /// An error specific to the display device
    DisplayError(&'static str),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "[{}:{}] {}",
            self.location.file, self.location.line, self.kind
        )
    }
}

impl fmt::Display for ErrorKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ErrorKind::Io(e) => write!(f, "IO error: {}", e),
            ErrorKind::InvalidPin(p) => write!(f, "Invalid pin: {}", p),
            ErrorKind::I2cNack => write!(f, "I2C NACK received"),
            ErrorKind::I2cTimeout => write!(f, "I2C timeout"),
            ErrorKind::DisplayError(s) => write!(f, "Display error: {}", s),
        }
    }
}

/// A type alias for Results that use the ferropin Error type
pub type Result<T> = std::result::Result<T, Error>;

/// Create a new error with automatic location tracking
///
/// # Example
///
/// ```
/// # use ferropin::error::{err, ErrorKind};
/// # fn might_fail() -> Result<(), ferropin::error::Error> {
/// return Err(err!(ErrorKind::I2cNack));
/// # }
/// ```
#[macro_export]
macro_rules! err {
    ($kind:expr) => {
        $crate::error::Error {
            kind: $kind,
            location: $crate::error::Location {
                file: file!(),
                line: line!(),
            },
        }
    };
}

/// Convert a Result<T, std::io::Error> into Result<T, Error>
///
/// This is useful when calling standard library functions that return std::io::Error
/// and you want to convert them to the ferropin error type.
///
/// # Example
///
/// ```
/// # use ferropin::error::try_io;
/// # use std::fs::File;
/// # fn open_device() -> Result<File, ferropin::error::Error> {
/// let file = try_io!(File::open("/dev/i2c-1"));
/// # Ok(file)
/// # }
/// ```
#[macro_export]
macro_rules! try_io {
    ($expr:expr) => {
        $expr.map_err(|e| $crate::err!($crate::error::ErrorKind::Io(e)))?
    };
}
