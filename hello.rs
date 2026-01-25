use std::ffi::{CStr, c_char};

unsafe extern "C" {
    pub fn lang() -> *const c_char;
}

fn main () {
    let cstr = unsafe { CStr::from_ptr(lang()) };
    println!("Hello world, from {}", cstr.to_str().unwrap());
}
