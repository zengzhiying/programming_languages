use std::{thread::{self}, time::Duration, sync::{Arc, Barrier}};

pub fn thread_example() {
    // 创建 1 个线程
    let handler = thread::spawn(|| {
        for i in 1..10 {
            println!("number {} from the spawned thread.", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    // 主线程
    for i in 1..5 {
        println!("number {} from the main.", i);
        thread::sleep(Duration::from_millis(1));
    }
    // 主线程结束后其余线程会被结束
    // 可以阻塞子线程，这样会让之后的操作在线程结束之后
    handler.join().unwrap();

    // 线程闭包中使用外部数据
    let v = vec![1, 2, 3];
    let handler = thread::spawn(move || {
        // 由于 Rust 无法判断 v 的生命周期，因此不允许在内部直接使用外部的变量
        // 需要使用 move 关键字移动所有权，这样线程内部就拥有的变量的所有权，外部也无法再使用
        println!("vector: {:?}", v);
    });
    // 无法再使用 v
    // println!("vector: {:?}", v);
    handler.join().unwrap();

    // 嵌套线程的情况，外面的线程退出后，里面创建的线程会继续运行
    // 直到 main 线程退出为止
    let t = thread::spawn(|| {
        // 继续创建线程
        thread::spawn(|| {
            loop {
                println!("sub thread.");
                thread::sleep(Duration::from_millis(10));
            }
        });
    });
    t.join().unwrap();
    println!("parent thread is finished.");

    // 这个时候子线程仍然会继续打印直到主线程结束
    thread::sleep(Duration::from_millis(100));
}

pub fn thread_barrier_example() {
    let mut handlers = Vec::with_capacity(6);
    // 使用引用计数设置屏障个数
    let barrier = Arc::new(Barrier::new(6));

    for _ in 0..6 {
        let b = barrier.clone();
        handlers.push(thread::spawn(move || {
            println!("thread start.");
            // 插入屏障可以使得所有线程都执行到 wait 后再执行后面的代码
            b.wait();
            println!("thread end.");
        }));
    }

    for handler in handlers {
        handler.join().unwrap();
    }
}
