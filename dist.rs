use std::ffi::{c_int, c_void};

unsafe extern "C" {
    fn qsort(
        base: *mut c_void,
        nmemb: usize,
        size: usize,
        compar: unsafe extern "C" fn(*const c_void, *const c_void) -> c_int,
    );
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
struct Point {
    x: f64,
    y: f64,
}

impl Point {
    fn dist_sq(&self) -> f64 {
        self.x * self.x + self.y * self.y
    }
}

unsafe extern "C" fn dist_cmp(a: *const c_void, b: *const c_void) -> c_int {
    let (da, db) = unsafe {
        let a = &*(a as *const Point);
        let b = &*(b as *const Point);
        (a.dist_sq(), b.dist_sq())
    };

    if da < db { -1 } else if da > db { 1 } else { 0 }
}

fn main() {
    let mut points = [
        Point { x: 3.0, y: 4.0 },   // d² = 25
        Point { x: 1.0, y: 1.0 },   // d² = 2
        Point { x: 0.0, y: 10.0 },  // d² = 100
        Point { x: 2.0, y: 0.0 },   // d² = 4
        Point { x: 6.0, y: 8.0 },   // d² = 100
    ];

    println!("Before: {:?}", points);

    unsafe {
        qsort(
            points.as_mut_ptr() as *mut c_void,
            points.len(),
            size_of::<Point>(),
            dist_cmp,
        );
    }

    println!("After:  {:?}", points);
}
