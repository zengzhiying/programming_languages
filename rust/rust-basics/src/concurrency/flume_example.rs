use std::thread;

// 高性能 mpmc 库：flume
// 相比庞大的 crossbeam 库更轻量，功能单一
pub fn flume_example() {
    // 异步无界通道
    let (tx, rx) = flume::unbounded();
    thread::spawn(move || {
        (0..10).for_each(|i| {
            tx.send(i).unwrap();
        });
    });

    // 通道关闭后得到求值结果
    let sum_value: u32 = rx.iter().sum();
    println!("sum value: {}", sum_value);

    // 有界通道
    let (tx, rx) = flume::bounded(100);
    let thread_num = 4;
    // 4 线程发送，4 线程接收
    let mut tx_handlers = Vec::new();
    for i in 0..thread_num {
        let tx_copy = tx.clone();
        let tx_handle = thread::spawn(move || {
            println!("tx thread {} send {}", i, i);
            tx_copy.send(i).unwrap();
        });
        tx_handlers.push(tx_handle);
        let rx_copy = rx.clone();
        thread::spawn(move || {
            // 多线程读取结果不是固定的，可能有些线程读取的数据多，有些线程没有读取到
            // 随着数据量的增大，读取会比较均匀
            for v in rx_copy {
                println!("rx thread {} received {}", i, v);
            }
        });
    }

    for tx_handle in tx_handlers {
        tx_handle.join().unwrap();
    }

    // 关闭发送通道，读取线程自动退出
    drop(tx);
}
