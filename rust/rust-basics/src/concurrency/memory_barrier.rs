
use std::{sync::{atomic::{AtomicBool, Ordering, AtomicUsize}, Arc}, thread::{JoinHandle, self}, hint};

// 内存屏障用法
static mut DATA: u64 = 0;
static READY: AtomicBool = AtomicBool::new(false);

// Ordering::Relaxed 最宽松的原则，对编译器和 CPU 不做限制，不保证顺序，通常用在单个原子操作中，比如计数器
// Ordering::Release 保证插入屏障之前的操作一定在屏障之前，不保证后面的重排顺序，也就是可能会排到屏障之前，通常用于写入
// Ordering::Acquire 保证插入屏障之后的操作一定在屏障之后，不保证前面的重排顺序，也就是可能会排到屏障之后，通常用于读取
// Ordering::AcqRel  相当于 Release 和 Acquire 的结合，保证前面的不会重排到后面同时后面的不会重排到前面
// Ordering::SeqCst  保证顺序一致性，是 AcqRel 的加强版，会刷新缓存，因此提供更强的保证，上面的其他都是针对原子操作来说的并不会保证多核缓存一致

fn reset() {
    unsafe {
        DATA = 0;
    }
    READY.store(false, Ordering::Relaxed);
}

fn producer() -> JoinHandle<()> {
    thread::spawn(|| {
        unsafe {
            DATA = 100;
        }
        // 保证之前的操作永远在屏障之前
        READY.store(true, Ordering::Release);
    })
}

fn consumer() -> JoinHandle<()> {
    thread::spawn(|| {
        // 保证之后的操作永远在屏障之后
        while !READY.load(Ordering::Acquire) {}
        assert_eq!(100, unsafe {
            DATA
        });
    })
}

pub fn memory_barrier_example() {
    for _ in 0..8 {
        reset();

        let p = producer();
        let c = consumer();

        p.join().unwrap();
        c.join().unwrap();
    }

    // Atomic 配合 SeqCst 提供顺序保证，或者当成锁使用
    let spinlock = Arc::new(AtomicUsize::new(1));
    let spinlock_clone = Arc::clone(&spinlock);
    let t = thread::spawn(move || {
        spinlock_clone.store(0, Ordering::SeqCst);
    });

    // 保证前面的线程完成操作
    while spinlock.load(Ordering::SeqCst) != 0 {
        hint::spin_loop();
    }

    t.join().unwrap();
    assert_eq!(0, spinlock.load(Ordering::SeqCst));
}