use std::sync::atomic::AtomicPtr;
use std::sync::atomic::Ordering::{Acquire, Release};

fn get_data() -> &'static Data {
    static PTR: AtomicPtr<Data> = AtomicPtr::new(std::ptr::null_mut());

    let mut p = PTR.load(Acquire);

    if p.is_null() {
        p = Box::into_raw(Box::new(generate_data()));
        if let Err(e) = PTR.compare_exchange(
            std::ptr::null_mut(), p, Release, Acquire
        ) {
            // Safety: p comes from Box::into_raw right above,
            // and wasn't shared with any other thread.
            drop(unsafe { Box::from_raw(p) });
            p = e;
        }
    }

    // Safety: p is not null and points to a properly initialized value.
    unsafe { &*p }
}

struct Data([u8; 100]);

fn generate_data() -> Data {
    Data([123; 100])
}

fn main() {
    println!("{:p}", get_data());
    println!("{:p}", get_data()); // Same address as before.
}
