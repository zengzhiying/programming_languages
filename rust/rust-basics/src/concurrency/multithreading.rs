use std::{thread, time::Duration};

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
}