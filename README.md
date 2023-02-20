This repository contains the code examples, data structures, and links from
[Rust Atomics and Locks](https://marabos.nl/atomics/).

The examples from chapters 1, 2, 3, and 8 can be found in [examples/](examples/).
The data structures from chapters 4, 5, 6, and 9 can be found in [src/](src/).

### Chapter 1 — Basics of Rust Concurrency

- [examples/ch1-01-hello.rs](examples/ch1-01-hello.rs)
- [examples/ch1-02-hello-join.rs](examples/ch1-02-hello-join.rs)
- [examples/ch1-03-spawn-closure.rs](examples/ch1-03-spawn-closure.rs)
- [examples/ch1-04-scoped-threads.rs](examples/ch1-04-scoped-threads.rs)
- [examples/ch1-05-rc.rs](examples/ch1-05-rc.rs)
- [examples/ch1-06-cell.rs](examples/ch1-06-cell.rs)
- [examples/ch1-07-refcell.rs](examples/ch1-07-refcell.rs)
- [examples/ch1-08-mutex.rs](examples/ch1-08-mutex.rs)
- [examples/ch1-09-sleep-before-unlock.rs](examples/ch1-09-sleep-before-unlock.rs)
- [examples/ch1-10-unlock-before-sleep.rs](examples/ch1-10-unlock-before-sleep.rs)
- [examples/ch1-11-thread-parking.rs](examples/ch1-11-thread-parking.rs)
- [examples/ch1-12-condvar.rs](examples/ch1-12-condvar.rs)

### Chapter 2 — Atomics

- [examples/ch2-01-stop-flag.rs](examples/ch2-01-stop-flag.rs)
- [examples/ch2-02-progress-reporting.rs](examples/ch2-02-progress-reporting.rs)
- [examples/ch2-03-progress-reporting-unpark.rs](examples/ch2-03-progress-reporting-unpark.rs)
- [examples/ch2-04-lazy-init.rs](examples/ch2-04-lazy-init.rs)
- [examples/ch2-05-fetch-add.rs](examples/ch2-05-fetch-add.rs)
- [examples/ch2-06-progress-reporting-multiple-threads.rs](examples/ch2-06-progress-reporting-multiple-threads.rs)
- [examples/ch2-07-statistics.rs](examples/ch2-07-statistics.rs)
- [examples/ch2-08-id-allocation.rs](examples/ch2-08-id-allocation.rs)
- [examples/ch2-09-id-allocation-panic.rs](examples/ch2-09-id-allocation-panic.rs)
- [examples/ch2-10-id-allocation-subtract-before-panic.rs](examples/ch2-10-id-allocation-subtract-before-panic.rs)
- [examples/ch2-11-increment-with-compare-exchange.rs](examples/ch2-11-increment-with-compare-exchange.rs)
- [examples/ch2-12-id-allocation-without-overflow.rs](examples/ch2-12-id-allocation-without-overflow.rs)
- [examples/ch2-13-lazy-one-time-init.rs](examples/ch2-13-lazy-one-time-init.rs)

### Chapter 3 — Memory Ordering

- [examples/ch3-01-relaxed.rs](examples/ch3-01-relaxed.rs)
- [examples/ch3-02-spawn-join.rs](examples/ch3-02-spawn-join.rs)
- [examples/ch3-03-total-modification-order.rs](examples/ch3-03-total-modification-order.rs)
- [examples/ch3-04-total-modification-order-2.rs](examples/ch3-04-total-modification-order-2.rs)
- [examples/ch3-05-out-of-thin-air.rs](examples/ch3-05-out-of-thin-air.rs)
- [examples/ch3-06-release-acquire.rs](examples/ch3-06-release-acquire.rs)
- [examples/ch3-07-release-acquire-unsafe.rs](examples/ch3-07-release-acquire-unsafe.rs)
- [examples/ch3-08-lock.rs](examples/ch3-08-lock.rs)
- [examples/ch3-09-lazy-init-box.rs](examples/ch3-09-lazy-init-box.rs)
- [examples/ch3-10-seqcst.rs](examples/ch3-10-seqcst.rs)
- [examples/ch3-11-fence.rs](examples/ch3-11-fence.rs)

### Chapter 4 — Building Our Own Spin Lock

- [src/ch4_spin_lock/s1_minimal.rs](src/ch4_spin_lock/s1_minimal.rs)
- [src/ch4_spin_lock/s2_unsafe.rs](src/ch4_spin_lock/s2_unsafe.rs)
- [src/ch4_spin_lock/s3_guard.rs](src/ch4_spin_lock/s3_guard.rs)

### Chapter 5 — Building Our Own Channels

- [src/ch5_channels/s1_simple.rs](src/ch5_channels/s1_simple.rs)
- [src/ch5_channels/s2_unsafe.rs](src/ch5_channels/s2_unsafe.rs)
- [src/ch5_channels/s3_checks.rs](src/ch5_channels/s3_checks.rs)
- [src/ch5_channels/s3_single_atomic.rs](src/ch5_channels/s3_single_atomic.rs)
- [src/ch5_channels/s4_types.rs](src/ch5_channels/s4_types.rs)
- [src/ch5_channels/s5_borrowing.rs](src/ch5_channels/s5_borrowing.rs)
- [src/ch5_channels/s6_blocking.rs](src/ch5_channels/s6_blocking.rs)

### Chapter 6 — Building Our Own “Arc”

- [src/ch6_arc/s1_basic.rs](src/ch6_arc/s1_basic.rs)
- [src/ch6_arc/s2_weak.rs](src/ch6_arc/s2_weak.rs)
- [src/ch6_arc/s3_optimized.rs](src/ch6_arc/s3_optimized.rs)

### Chapter 7 — Understanding the Processor

- https://godbolt.org/

### Chapter 8 — Operating System Primitives

- [examples/ch8-01-futex.rs](examples/ch8-01-futex.rs)

### Chapter 9 — Building Our Own Locks

- [src/ch9_locks/mutex_1.rs](src/ch9_locks/mutex_1.rs)
- [src/ch9_locks/mutex_2.rs](src/ch9_locks/mutex_2.rs)
- [src/ch9_locks/mutex_3.rs](src/ch9_locks/mutex_3.rs)
- [src/ch9_locks/condvar_1.rs](src/ch9_locks/condvar_1.rs)
- [src/ch9_locks/condvar_2.rs](src/ch9_locks/condvar_2.rs)
- [src/ch9_locks/rwlock_1.rs](src/ch9_locks/rwlock_1.rs)
- [src/ch9_locks/rwlock_2.rs](src/ch9_locks/rwlock_2.rs)
- [src/ch9_locks/rwlock_3.rs](src/ch9_locks/rwlock_3.rs)

### Chapter 10 — Ideas and Inspiration

- [Wikipedia article on semaphores](https://en.wikipedia.org/wiki/Semaphore_(programming))
- [Stanford University course notes on semaphores](https://see.stanford.edu/materials/icsppcs107/23-Concurrency-Examples.pdf)
- [Wikipedia article on the read-copy-update pattern](https://en.wikipedia.org/wiki/Read-copy-update)
- [LWN article "What is RCU, Fundamentally?"](https://lwn.net/Articles/262464/)
- [Wikipedia article on non-blocking linked lists](https://en.wikipedia.org/wiki/Non-blocking_linked_list)
- [LWN article "Using RCU for Linked Lists—A Case Study"](https://lwn.net/Articles/610972/)
- [Notes on the implementation of Windows SRW locks](https://github.com/rust-lang/rust/issues/93740#issuecomment-1064139337)
- [A Rust implementation of queue-based locks](https://github.com/kprotty/usync)
- [WebKit blog post, "Locking in WebKit"](https://webkit.org/blog/6161/locking-in-webkit/)
- [Documentation of the `parking_lot` crate](https://docs.rs/parking_lot)
- [Wikipedia article on Linux's Seqlock](https://en.wikipedia.org/wiki/Seqlock)
- [Rust RFC 3301, `AtomicPerByte`](https://rust.tf/rfc3301)
- [Documentation of the `seqlock` crate](https://docs.rs/seqlock)

### License

You may use all code in this repository for any purpose.

Attribution is appreciated, but not required.
An attribution usually includes the book title, author, publisher, and ISBN.
For example: "_Rust Atomics and Locks_ by Mara Bos (O’Reilly). Copyright 2023 Mara Bos, 978-1-098-11944-7."
