use std::sync::atomic::AtomicU64;
use std::sync::atomic::Ordering::Relaxed;
use std::thread;
use std::time::Duration;

fn get_x() -> u64 {
    static X: AtomicU64 = AtomicU64::new(0);
    let mut x = X.load(Relaxed);
    if x == 0 {
        x = calculate_x();
        X.store(x, Relaxed);
    }
    x
}

fn calculate_x() -> u64 {
    thread::sleep(Duration::from_secs(1));
    123
}

fn main() {
    dbg!(get_x());
    dbg!(get_x());
}
