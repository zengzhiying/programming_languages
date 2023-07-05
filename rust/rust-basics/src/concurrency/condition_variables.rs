use std::{sync::{Arc, Mutex, Condvar}, thread};


pub fn condition_variables_example() {
    let pair = Arc::new((Mutex::new(false), Condvar::new()));
    let pair_copy1 = pair.clone();
    let pair_copy2 = pair.clone();

    thread::spawn(move || {
        println!("thread 1 started.");
        let (lock, cvar) = &*pair_copy1;
        let mut started = lock.lock().unwrap();
        println!("thread 1 acquired lock.");
        if !*started {
            println!("thread 1 modified status.");
            *started = true;
        }
        // 通知主线程 
        cvar.notify_one();
        println!("thread 1 exit.");
    });

    thread::spawn(move || {
        println!("thread 2 started.");
        let (lock, cvar) = &*pair_copy2;
        let mut started = lock.lock().unwrap();
        println!("thread 2 acquired lock.");
        if !*started {
            println!("thread 2 modified status.");
            *started = true;
        }
        // 通知主线程
        cvar.notify_one();
        println!("thread 2 exit.");
    });

    let (lock, cvar) = &*pair;
    let mut started = lock.lock().unwrap();
    while !*started {
        println!("waiting.");
        // 释放锁并等待子线程通知
        started = cvar.wait(started).unwrap();
    }

    println!("main.");
}
