//! Error handling across ffi boundary
//!
//! This module provides error handling across
//! the ffi boundary using errno and strerror

use std::ffi::{c_char, c_int, CStr, CString};

#[cfg(not(target_os = "windows"))]
unsafe extern "C" {
    fn open(path: *const c_char, flags: c_int) -> c_int;
    fn close(fd: c_int) -> c_int;
}

#[cfg(target_os = "windows")]
unsafe extern "C" {
    fn _open(path: *const c_char, flags: c_int) -> c_int;
    fn _close(fd: c_int) -> c_int;
}

unsafe extern "C" {
    fn strerror(errnum: c_int) -> *const c_char;

    #[cfg(target_os = "linux")]
    fn __errno_location() -> *mut c_int;

    #[cfg(target_os = "macos")]
    fn __error() -> *mut c_int;

    #[cfg(target_os = "windows")]
    fn _errno() -> *mut c_int;
}

fn errno() -> c_int {
    #[cfg(target_os = "linux")]
    unsafe { *__errno_location() }

    #[cfg(target_os = "macos")]
    unsafe { *__error() }

    #[cfg(target_os = "windows")]
    unsafe { *_errno() }
}

fn error_string(errnum: c_int) -> String {
    unsafe { CStr::from_ptr(strerror(errnum)).to_string_lossy().into_owned() }
}

const O_RDONLY: c_int = 0;

#[cfg(not(target_os = "windows"))]
fn open_file(path: *const c_char, flags: c_int) -> c_int {
    unsafe { open(path, flags) }
}

#[cfg(target_os = "windows")]
fn open_file(path: *const c_char, flags: c_int) -> c_int {
    unsafe { _open(path, flags) }
}

#[cfg(not(target_os = "windows"))]
fn close_file(fd: c_int) -> c_int {
    unsafe { close(fd) }
}

#[cfg(target_os = "windows")]
fn close_file(fd: c_int) -> c_int {
    unsafe { _close(fd) }
}

fn main() {
    #[cfg(not(target_os = "windows"))]
    let path = CString::new("/nonexistent/file.txt").unwrap();

    #[cfg(target_os = "windows")]
    let path = CString::new("C:\\nonexistent\\file.txt").unwrap();

    let fd = open_file(path.as_ptr(), O_RDONLY);

    if fd == -1 {
        let err = errno();
        println!("open() failed: {} (errno={})", error_string(err), err);
    } else {
        println!("open() succeeded: fd={}", fd);
        close_file(fd);
    }
}
