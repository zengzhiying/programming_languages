use std::{sync::mpsc, thread, time::Duration};


// mpsc: multiple producer, single consumer
pub fn mpsc_example() {
    // mspc：同样包括单生产者和单消费者
    // 默认情况下通道是异步的，在发送消息时不会阻塞，且长度不受限制
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

    // 当发送者全部被 drop 后，接收者才会退出循环，关闭通常都是在编译期实现的无需显式关闭
    // 通道本身是 FIFO，但是多个线程发送的情况下由于发送顺序不确定，因此接收也不存在顺序关系
    for v in rx {
        println!("reveive {}", v);
    }

    // 同步通道：发送消息是阻塞的，只有被消费者接收消息后才会解除阻塞
    let (tx, rx) = mpsc::sync_channel(0);
    let handle = thread::spawn(move || {
        println!("thread begin.");
        tx.send(1).unwrap();
        println!("thread end.");
    });

    println!("main begin.");
    thread::sleep(Duration::from_secs(3));
    println!("main end.");

    println!("recv: {}", rx.recv().unwrap());
    handle.join().unwrap();

    // 为了避免异步通道使用不当导致内存溢出，可以使用带缓存的同步通道
    let (tx, rx) = mpsc::sync_channel(10);

    thread::spawn(move || {
        for i in 0..10 {
            tx.send(i).unwrap();
        }
    });

    for v in rx {
        println!("recv: {}", v);
    }

    enum Fruit {
        Apple(u8),
        Orange(String),
    }

    // channel 只能传输 1 种类型的数据，如果想传输多种数据可以使用结构体或枚举配合判断逻辑间接实现
    // 由于内存对齐，也有可能会造成内存上的浪费
    let (tx, rx) = mpsc::channel();

    tx.send(Fruit::Apple(3)).unwrap();
    tx.send(Fruit::Orange("sweet".to_string())).unwrap();

    // 注意不能使用 for x in rx 形式，因为生产者没有关闭，因此不会自动退出
    for _ in 0..2 {
        match rx.recv().unwrap() {
            Fruit::Apple(v) => {
                println!("receved {} apples", v);
            },
            Fruit::Orange(v) => {
                println!("received {} oranges", v);
            },
        }
    }

    // 注意 tx 必须关闭 for x in rx 才会退出
    // 关闭的条件是持有 tx 的全部线程都结束或者 tx 退出其作用域被释放
    let (tx, rx) = mpsc::channel();

    // 启动 4 个线程
    for i in 0..4 {
        let tx_clone = tx.clone();
        thread::spawn(move || {
            tx_clone.send(i).unwrap();
            println!("thrad {} finished.", i);
        });
    }

    // 由于 tx 本身仍然没有被释放，因此需要显式释放使其关闭掉
    drop(tx);

    // 如果没有上面的 drop 这里会无限阻塞
    for x in rx {
        println!("Recv: {}", x);
    }
}
