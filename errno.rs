//! Error handling across ffi boundary
//!
//! This module provides error handling across
//! the ffi boundary using errno and strerror
//! Only works on Linux

use std::ffi::{c_char, c_int, CStr, CString};

unsafe extern "C" {
    fn open(path: *const c_char, flags: c_int) -> c_int;
    fn close(fd: c_int) -> c_int;
    fn strerror(errnum: c_int) -> *const c_char;
    fn __errno_location() -> *mut c_int; // Linux-specific
}

fn errno() -> c_int {
    unsafe { *__errno_location() }
}

fn error_string(errnum: c_int) -> String {
    unsafe { CStr::from_ptr(strerror(errnum)).to_string_lossy().into_owned() }
}

const O_RDONLY: c_int = 0;

fn main() {
    let path = CString::new("/nonexistent/file.txt").unwrap();

    let fd = unsafe { open(path.as_ptr(), O_RDONLY) };

    if fd == -1 {
        let err = errno();
        println!("open() failed: {} (errno={})", error_string(err), err);
    } else {
        println!("open() succeeded: fd={}", fd);
        unsafe { close(fd) };
    }
}
