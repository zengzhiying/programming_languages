use std::{sync::{Arc, Mutex, Condvar}, thread, time::Duration};


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


    // 还可以实现主线程和子线程交替打印内容的效果
    // 标记变量，使用 Arc/Mutex 实现
    let flag = Arc::new(Mutex::new(false));
    // 条件变量，使用 Arc/Condvar 实现
    let cond = Arc::new(Condvar::new());
    // 克隆变量提供线程使用
    let t_flag = flag.clone();
    let t_cond = cond.clone();

    let handle = thread::spawn(move || {
        let mut m = *t_flag.lock().unwrap();
        let mut counter = 0;

        while counter < 3 {
            // 这里必然进入循环
            while !m {
                // 加锁后释放锁并等待主线程通知
                m = *t_cond.wait(t_flag.lock().unwrap()).unwrap();
            }

            // 这里主线程进入下一次循环后 sleep
            // 这里一定会先执行
            m = false;
            *t_flag.lock().unwrap() = false;
            counter += 1;
            println!("thread counter: {}", counter);
        }
    });

    let mut counter = 0;
    loop {
        thread::sleep(Duration::from_millis(1000));
        // 因为线程再等待条件变量，所以这里最终一定可以加锁成功
        *flag.lock().unwrap() = true;
        counter += 1;
        if counter > 3 {
            break;
        }
        println!("main counter: {}", counter);
        // 通知子线程，然后进入下一次循环会自动释放掉当前的锁
        cond.notify_one();
    }

    handle.join().unwrap();
    // 主线程进行最后一次循环，因此 flag 一定为 true 且处于未加锁状态
    println!("main flag: {:?}", flag);
}
