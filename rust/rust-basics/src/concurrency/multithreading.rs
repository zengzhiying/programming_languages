use std::{thread::{self, LocalKey}, time::Duration, sync::{Arc, Barrier}, cell::RefCell};

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

// 线程局部变量也可以放在结构体中
struct LocalVar;
impl LocalVar {
    thread_local! {
        static VALUE: RefCell<u32> = RefCell::new(1);
    }
}

// 其他结构体中也可以引用
struct LocalVarRef {
    value: &'static LocalKey<RefCell<u32>>
}
impl LocalVarRef {
    fn new() -> Self {
        Self { value: &LocalVar::VALUE }
    }
}

pub fn threading_local_variable() {
    // 初始化线程局部变量 生命周期为 'static
    thread_local! {static VALUE: RefCell<u32> = RefCell::new(1)};
    // 通过 with 来获取变量值或进行赋值
    VALUE.with(|f| {
        assert_eq!(*f.borrow(), 1);
        *f.borrow_mut() = 2;
    });

    // 每个线程开始时都会拿到设置的初始值 1
    // 多个线程的设置都是局部生效，相互之间不影响
    // 上面的 with 相当于主线程使用局部变量
    let t = thread::spawn(|| {
        VALUE.with(|f| {
            assert_eq!(*f.borrow(), 1);
            *f.borrow_mut() = 3;
        });
    });

    t.join().unwrap();

    // 外部再次使用时，拿到的仍然是刚才设置的局部值 2
    // 多个线程的局部变量结果是无法汇总的
    VALUE.with(|f| {
        assert_eq!(*f.borrow(), 2);
    });

    // 也可以使用结构体实现的线程局部变量
    LocalVar::VALUE.with(|f| {
        println!("{:?}", f);
        *f.borrow_mut() += 1;
    });

    let r = LocalVarRef::new();
    r.value.with(|x| {
        // 因为都是在 1 个线程中因此值为 2
        println!("{:?}", x);
    });

    println!("YES");
}
