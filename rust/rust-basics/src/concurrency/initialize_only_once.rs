use std::{sync::Once, thread};

static mut VAL: usize = 0;
static INIT: Once = Once::new();

pub fn initialize_only_once_example() {
    let thread1 = thread::spawn(|| {
        INIT.call_once(|| {
            println!("thread 1 called.");
            unsafe {
                VAL = 1;
            };
        });
    });

    let thread2 = thread::spawn(|| {
        INIT.call_once(|| {
            println!("thread 2 called.");
            unsafe {
                VAL = 2;
            };
        });
    });

    thread1.join().unwrap();
    thread2.join().unwrap();

    println!("{}", unsafe {
        VAL
    });
}
