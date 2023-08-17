use std::{sync::Arc, thread, time::Duration};

use tokio::sync::Semaphore;

// 推荐使用 tokio 第三方库实现信号量
// https://github.com/tokio-rs/tokio
#[tokio::main]
pub async fn tokio_semaphore_example() {
    let semaphore = Arc::new(Semaphore::new(3));
    let mut handlers = Vec::new();

    for i in 0..6 {
        let permit = semaphore.clone().acquire_owned().await.unwrap();
        handlers.push(tokio::spawn(async move {
            println!("task: {}", i);
            // 因为信号量的最大容量是3，因此先输出 3 条，然后停顿后再输出后 3 条
            thread::sleep(Duration::from_secs(1));
            // 这里需要使用一下，保证信号量的副本被回收
            drop(permit);
            // 或者任意赋值一下，转移外部所有权，效果一样
            // let _v = permit;
        }));
    }

    for handle in handlers {
        handle.await.unwrap();
    }
}