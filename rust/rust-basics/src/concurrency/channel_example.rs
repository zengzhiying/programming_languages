use std::{sync::mpsc, thread, time::Duration};


// mpsc: multiple producer, single consumer
pub fn mpsc_example() {
    // mspc 同样包括单生产者和单消费者
    let (tx, rx) = mpsc::channel();

    // 线程发送消息
    thread::spawn(move || {
        // 往通道发送内容 类型会自动推导
        // 之后则只能使用该类型的值
        // 如果接收者不存在则会出现错误，需要单独处理
        tx.send(1).unwrap();
    });

    // 主线程接收消息，recv 会阻塞到有消息或者 channel 被关闭
    // channel 被关闭时会返回错误 unwrap 会 panic，需要单独处理错误
    println!("receive {}", rx.recv().unwrap());

    // 不阻塞的方法 try_recv 不存在消息时会直接返回
    // 这里通常是 Err(Disconnected) 因为线程已经结束，通道被关闭
    println!("receive {:?}", rx.try_recv());


    // 如果发送的数据实现了 Copy trait 则数据会复制一份进行传输
    // 如果没有实现 Copy trait 则会直接将所有权转移到接收端
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let s = String::from("Hello");
        tx.send(s).unwrap();
        // 所有权已被转移 无法再使用
        // println!("s: {}", s);
    });

    let received = rx.recv().unwrap();
    println!("receive string {}", received);

    // 循环发送与循环接收
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let vals = vec![
            String::from("a"),
            String::from("b"),
            String::from("c")
        ];

        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    // 使用循环 接收者收到错误时会直接退出循环
    for rec in rx {
        println!("receive {}", rec);
    }


    // 直接使用多发送者
    let (tx, rx) = mpsc::channel();
    let tx1 = tx.clone();
    thread::spawn(move || {
        tx.send(1).unwrap();
        tx.send(2).unwrap();
        tx.send(3).unwrap();
    });

    thread::spawn(move || {
        tx1.send(4).unwrap();
        tx1.send(5).unwrap();
        tx1.send(6).unwrap();
    });

    // 当发送者全部被 drop 后，接收者才会退出循环
    // 通道本身是 FIFO，但是多个线程发送的情况下由于发送顺序不确定，因此接收也不存在顺序关系
    for v in rx {
        println!("reveive {}", v);
    }
}
