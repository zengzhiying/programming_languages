use std::{sync::{Mutex, Arc, RwLock}, thread};

pub fn mutex_example() {
    // 数据放到互斥锁内，表示使用互斥锁管理
    let m = Mutex::new(5);
    {
        let mut num = m.lock().unwrap();
        *num+=1;
        // 用完在离开作用域后锁会自动释放，但是如果紧接着用必须手动释放，否则会死锁
        drop(num);

        let mut num1 = m.lock().unwrap();
        *num1+=1;

        // drop(num1);
        // 如果没 drop 这里打印是 locked 默认在退出作用域 drop
        // 如果手动 drop 这里打印是具体的数据
        println!("m: {:?}", m);
    }

    // 这里默认会自动解锁
    println!("m: {:?}", m);

    // 多线程使用锁
    // 注意引用计数只能使用 Arc，Rc 不能用于多线程环境
    // 单线程使用 Ac/RefCell 实现内部可变性，多线程则使用 Arc/Mutex 实现内部可变性
    let counter = Arc::new(Mutex::new(0));
    let mut handlers = Vec::new();

    for _ in 0..10 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();
            *num += 1;
        });
        handlers.push(handle);
    }

    for handle in handlers {
        handle.join().unwrap();
    }

    println!("counter: {}", *counter.lock().unwrap());

    let _v = counter.lock().unwrap();

    // 删除后下面才可以正常获取锁
    drop(_v);

    // 可以使用 try_lock 非阻塞方式获取锁
    let l = counter.try_lock();
    match l {
        Ok(mut lk) => {
            *lk+=1;
            println!("成功获取锁: {}", *lk);
        },
        Err(err) => println!("获取锁错误: {}", err),
    }

    println!("counter: {:?}", counter);
    
}

pub fn rw_mutex_example() {
    // 读写锁允许多个并发读，只允许 1 个写
    let rw_lock = RwLock::new(5);

    {
        // 只允许 1 个写
        let mut w = rw_lock.write().unwrap();
        *w += 1;
        assert_eq!(*w, 6);
        // 这时候如果使用读锁会直接 panic，写的时候不允许加读锁
    }
    // 锁超出生命周期被释放
    {
        // 允许多个读
        let r1 = rw_lock.read().unwrap();
        let r2 = rw_lock.read().unwrap();
        assert_eq!(*r1, 6);
        assert_eq!(*r2, 6);
    }
}