use std::ffi::{CStr, CString, c_char};

unsafe extern "C" {
    fn c_function(s: *const c_char) -> *const c_char;
}

#[unsafe(no_mangle)]
pub extern "C" fn rust_callback(s: *const c_char) -> *const c_char {
    let input = unsafe { CStr::from_ptr(s) }.to_str().unwrap();
    CString::new(format!("{input} Rust")).unwrap().into_raw()
}

fn main() {
    let input = CString::new("Hello from").unwrap();
    let result = unsafe { c_function(input.as_ptr()) };
    let output = unsafe { CStr::from_ptr(result) }.to_str().unwrap();
    println!("{output}");
}
