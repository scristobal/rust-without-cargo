use std::ffi::{c_char, c_int, c_long, c_ulong, c_void, CStr};
use std::ptr;

// mini-gmp struct (matches __mpz_struct in mini-gmp.h)
#[repr(C)]
struct MpzStruct {
    alloc: c_int,
    size: c_int,
    d: *mut c_ulong,
}

unsafe extern "C" {
    fn mpz_init(x: *mut MpzStruct);
    fn mpz_clear(x: *mut MpzStruct);
    fn mpz_set_si(x: *mut MpzStruct, val: c_long);
    fn mpz_mul(r: *mut MpzStruct, a: *const MpzStruct, b: *const MpzStruct);
    fn mpz_get_str(buf: *mut c_char, base: c_int, x: *const MpzStruct) -> *mut c_char;
    fn free(ptr: *mut c_void);
}

fn mpz_to_string(x: &MpzStruct) -> String {
    unsafe {
        let ptr = mpz_get_str(ptr::null_mut(), 10, x);
        let s = CStr::from_ptr(ptr).to_string_lossy().into_owned();
        free(ptr as *mut _);
        s
    }
}

const N: c_long = 50;

fn main() {
    unsafe {
        let mut result = std::mem::zeroed::<MpzStruct>();
        let mut i = std::mem::zeroed::<MpzStruct>();

        mpz_init(&mut result);
        mpz_init(&mut i);

        mpz_set_si(&mut result, 1);

        for n in 2..=N {
            mpz_set_si(&mut i, n);
            mpz_mul(&mut result, &result, &i);
        }

        println!("{}! = {}",N, mpz_to_string(&result));

        mpz_clear(&mut result);
        mpz_clear(&mut i);
    }
}
