use std::{sync::atomic::{AtomicU64, Ordering}, thread::{JoinHandle, self}, time::Instant, ops::Sub};

const N_TIMES: u64 = 10000000;
const N_THREADS: usize = 8;

static R: AtomicU64 = AtomicU64::new(0);

fn add_n_times(n: u64) -> JoinHandle<()> {
    thread::spawn(move || {
        for _ in 0..n {
            // Relaxed 是最弱的保证只能保证变量的原子性
            R.fetch_add(1, Ordering::Relaxed);
        }
    })
}

pub fn atomic_example() {
    let s = Instant::now();
    let mut threads = Vec::with_capacity(N_THREADS);
    for _ in 0..N_THREADS {
        threads.push(add_n_times(N_TIMES));
    }
    for t in threads {
        t.join().unwrap();
    }

    let num = N_TIMES * N_THREADS as u64;

    assert_eq!(num, R.load(Ordering::Relaxed));
    println!("{} add time: {:?}", num, Instant::now().sub(s));

    // Atomic 变量具有内部可变性，因此无需声明为 mut
    let v = AtomicU64::new(0);
    v.fetch_add(2, Ordering::Relaxed);
    assert_eq!(2, v.load(Ordering::Relaxed));
}