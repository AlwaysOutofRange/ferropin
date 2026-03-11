use std::{arch::asm, os::fd::RawFd};

// System call numbers for ARM64 (aarch64)
const SYS_OPENAT: i64 = 56;
const SYS_IOCTL: i64 = 29;
const SYS_CLOSE: i64 = 57;
const SYS_WRITE: i64 = 64;
const SYS_READ: i64 = 63;

// File descriptor for current working directory (used with openat)
const AT_FDCWD: i64 = -100;

/// I2C slave mode ioctl request
pub const I2C_SLAVE: u64 = 0x0703;
/// Open file for reading and writing
pub const O_RDWR: i64 = 2;

/// Open a file using the openat system call
///
/// # Arguments
///
/// * `path` - Pointer to the null-terminated path string
/// * `flags` - Open flags (e.g., O_RDWR)
///
/// # Returns
///
/// File descriptor on success, negative error code on failure
pub fn open(path: *const u8, flags: i64) -> i64 {
    unsafe {
        let ret: i64;
        asm!(
            "svc #0",
            in("x8") SYS_OPENAT,
            in("x0") AT_FDCWD,
            in("x1") path,
            in("x2") flags,
            in("x3") 0i64,
            lateout("x0") ret,
            options(nostack),
        );
        ret
    }
}

/// Write data to a file descriptor
///
/// # Arguments
///
/// * `fd` - File descriptor to write to
/// * `buf` - Pointer to the data buffer
/// * `len` - Number of bytes to write
///
/// # Returns
///
/// Number of bytes written on success, negative error code on failure
pub fn write(fd: RawFd, buf: *const u8, len: usize) -> i64 {
    unsafe {
        let ret: i64;
        asm!(
            "svc #0",
            in("x8") SYS_WRITE,
            in("x0") fd as i64,
            in("x1") buf,
            in("x2") len,
            lateout("x0") ret,
            options(nostack),
        );
        ret
    }
}

/// Read data from a file descriptor
///
/// # Arguments
///
/// * `fd` - File descriptor to read from
/// * `buf` - Pointer to the buffer where data will be stored
/// * `len` - Maximum number of bytes to read
///
/// # Returns
///
/// Number of bytes read on success, negative error code on failure
pub fn read(fd: RawFd, buf: *mut u8, len: usize) -> i64 {
    unsafe {
        let ret: i64;
        asm!(
            "svc #0",
            in("x8") SYS_READ,
            in("x0") fd as i64,
            in("x1") buf,
            in("x2") len,
            lateout("x0") ret,
            options(nostack),
        );
        ret
    }
}

/// Perform an ioctl request on a file descriptor
///
/// # Arguments
///
/// * `fd` - File descriptor to perform ioctl on
/// * `request` - The ioctl request number
/// * `arg` - Argument pointer (often cast to u64)
///
/// # Returns
///
/// 0 on success, negative error code on failure
pub fn ioctl(fd: RawFd, request: u64, arg: u64) -> i64 {
    unsafe {
        let ret: i64;
        asm!(
            "svc #0",
            in("x8") SYS_IOCTL,
            in("x0") fd as i64,
            in("x1") request,
            in("x2") arg,
            lateout("x0") ret,
            options(nostack)
        );
        ret
    }
}

/// Close a file descriptor
///
/// # Arguments
///
/// * `fd` - File descriptor to close
///
/// # Returns
///
/// 0 on success, negative error code on failure
pub fn close(fd: RawFd) -> i64 {
    unsafe {
        let ret: i64;
        asm!(
            "svc #0",
            in("x8") SYS_CLOSE,
            in("x0") fd as i64,
            lateout("x0") ret,
            options(nostack)
        );
        ret
    }
}
