use std::sync::atomic::AtomicU64;
use std::sync::atomic::Ordering::Relaxed;

fn get_key() -> u64 {
    static KEY: AtomicU64 = AtomicU64::new(0);
    let key = KEY.load(Relaxed);
    if key == 0 {
        let new_key = generate_random_key();
        match KEY.compare_exchange(0, new_key, Relaxed, Relaxed) {
            Ok(_) => new_key,
            Err(k) => k,
        }
    } else {
        key
    }
}

fn generate_random_key() -> u64 {
    123
    // TODO
}

fn main() {
    dbg!(get_key());
    dbg!(get_key());
    dbg!(get_key());
}
