use std::{rc::Rc, sync::Arc, thread::{self, JoinHandle}};


pub fn rc_example() {
    // 由于 Rust 的所有权机制导致 1 个值只能有 1 个所有者，但是在某些情况下一个数据资源需要有多个所有者，
    // 这个时候就可以使用 Rc（Reference counting），也就是引用计数的智能指针来实现，
    // Rc 可以在堆上分配对象提供程序多个部分使用，而这多个部分并不是同时结束的

    let s = String::from("hello");
    // s 所有权被转移
    let a = Box::new(s);
    // 再次使用会报错
    // let b = Box::new(s);
    println!("str: {}", a);

    // 使用 Rc 解决所有权转移问题
    let s = String::from("hello");
    // s 所有权同样被转移
    let a = Rc::new(s);
    // 查看引用计数
    println!("rc num: {}", Rc::strong_count(&a));
    // 但是可以使用 clone 增加引用计数，注意并不拷贝数据
    // 因为传参是引用，只会增加引用计数，因此效率非常高
    let b = Rc::clone(&a);
    println!("str a: {}, str b: {}", a, b);
    // 不管哪个计数都是 2，也就是指向同一个引用的所有变量都可以使用
    println!("rc num: {}, {}", Rc::strong_count(&a), Rc::strong_count(&b));

    // 测试引用计数的变化
    {
        // 引用计数为 3
        let c = Rc::clone(&b);
        println!("c {} rc num: {}", c, Rc::strong_count(&a));
    }

    // 自动释放后降为 2，因为 Rc<T> 实现了 Drop trait，所以释放变量 c 的同时引用计数也会减少
    println!("rc num: {}", Rc::strong_count(&a));

    drop(b);
    // 引用计数 -1
    println!("rc num: {}", Rc::strong_count(&a));

    // 当 a, b 超出作用域后，引用计数会归零，但是这个无法在程序中看到

}

pub fn arc_example() {
    // 多线程引用计数智能指针 Arc，可以在多个线程间安全地共享数据
    // 注意 Arc 也是只读的，独立使用无法修改，如果要修改变量需要配合 Cell/RelCell 来实现
    let s = Arc::new(String::from("Multithreading"));
    let mut vec: Vec<JoinHandle<()>> = Vec::new();
    for _ in 0..8 {
        let v = Arc::clone(&s);
        // 使用 move 关键字将所有权转移到闭包内
        let handle = thread::spawn(move || {
            println!("v: {}", v);
        });
        vec.push(handle);
    }
    // 这里的数字是不确定的, 取决于多少线程还没有执行完
    println!("arc num: {}", Arc::strong_count(&s));
    for _ in 0..8 {
        let handle = vec.pop().unwrap();
        let res = handle.join();
        match res {
            Ok(_) => {println!("Thread OK!")},
            Err(e) => {println!("err: {:?}", e);},
        }
    }
}
