use std::fmt;

#[doc = "Location where an error occurred"]
#[derive(Debug)]
pub struct Location {
    pub file: &'static str,
    pub line: u32,
}

#[doc = "Main error type for the ferropin library"]
#[derive(Debug)]
pub struct Error {
    pub kind: ErrorKind,
    pub location: Location,
}

#[doc = "Different categories of errors that can occur"]
#[derive(Debug)]
pub enum ErrorKind {
    #[doc = "I/O error occurred"]
    Io(std::io::Error),
    #[doc = "Invalid pin number was specified"]
    InvalidPin(u8),
    #[doc = "I2C device did not acknowledge a transmission"]
    I2cNack,
    #[doc = "I2C operation timed out"]
    I2cTimeout,
    #[doc = "Error specific to the display device"]
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

pub type Result<T> = std::result::Result<T, Error>;

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

#[macro_export]
macro_rules! try_io {
    ($expr:expr) => {
        $expr.map_err(|e| $crate::err!($crate::error::ErrorKind::Io(e)))?
    };
}
