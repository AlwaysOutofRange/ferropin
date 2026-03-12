use std::fmt;

#[derive(Debug)]
pub struct Location {
    pub file: &'static str,
    pub line: u32,
}

#[derive(Debug)]
pub struct Error {
    pub kind: ErrorKind,
    pub location: Location,
}

#[derive(Debug)]
pub enum ErrorKind {
    Io(std::io::Error),
    InvalidPin(u8),
    I2cNack,
    I2cTimeout,
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

impl std::error::Error for Error {}

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

pub type Result<T> = std::result::Result<T, Error>;

/// Create an error with automatic file/line tracking.
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

/// Convert `std::io::Error` into our error type and propagate with `?`.
#[macro_export]
macro_rules! try_io {
    ($expr:expr) => {
        $expr.map_err(|e| $crate::err!($crate::error::ErrorKind::Io(e)))?
    };
}
