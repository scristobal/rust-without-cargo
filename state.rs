use std::ffi::c_int;
use std::sync::atomic::{AtomicI32, Ordering};
use std::thread;

unsafe extern "C" {
    fn increment() -> c_int; 
    static global_counter: AtomicI32;
}

fn main() {
    let handles: [_; 5] = std::array::from_fn(|id| {
        thread::spawn(move || {
                let global = unsafe { global_counter.load(Ordering::SeqCst) };
                println!("Thread {id}: Start, global={global}");

                for _ in 0..10 {
                    let local = unsafe { increment() };
                    println!("Thread {id}: Running, local={local}");
                }

                let global = unsafe { global_counter.load(Ordering::SeqCst) };
                println!("Thread {id}: Done, global={global}");
        })
    });

    for h in handles {
        h.join().unwrap();
    }
}
