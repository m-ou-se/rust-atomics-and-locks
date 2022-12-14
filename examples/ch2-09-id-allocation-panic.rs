use std::sync::atomic::AtomicU32;
use std::sync::atomic::Ordering::Relaxed;

// This version is problematic.
fn allocate_new_id() -> u32 {
    static NEXT_ID: AtomicU32 = AtomicU32::new(0);
    let id = NEXT_ID.fetch_add(1, Relaxed);
    assert!(id < 1000, "too many IDs!");
    id
}

fn main() {
    dbg!(allocate_new_id()); // This will produce a zero.

    for _ in 1..1000 {
        allocate_new_id(); // 1 through 999.
    }

    println!("overflowing the counter... (this might take a few hours)");

    std::panic::set_hook(Box::new(|_| {}));

    for _ in 1000..=u32::MAX {
        let _ = std::panic::catch_unwind(|| allocate_new_id());
    }

    println!("overflowed!");

    dbg!(allocate_new_id()); // ⚠️ This will produce zero again. ⚠️
}
