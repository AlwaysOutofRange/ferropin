//! Direct ARM64 syscall wrappers (no libc).

use std::{arch::asm, os::fd::RawFd};

const SYS_OPENAT: i64 = 56;
const SYS_IOCTL: i64 = 29;
const SYS_CLOSE: i64 = 57;
const SYS_WRITE: i64 = 64;
const SYS_READ: i64 = 63;

const AT_FDCWD: i64 = -100;

pub const I2C_SLAVE: u64 = 0x0703;
pub const O_RDWR: i64 = 2;

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
