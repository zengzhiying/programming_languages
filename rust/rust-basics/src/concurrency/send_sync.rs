use std::thread;


pub fn send_sync_example() {
    // Rc 不能用于多线程而 Arc 可用于多线程的原因是因为 Arc 实现了 Send 和 Sync trait
    // 实现 Send trait 的类型可以在线程间安全传递所有权，实现 Sync trait 的类型可以在线程间通过共享引用
    // 对于结构体来说，只要其内部的成员全部实现了 Send 或 Sync，那么结构体也就自动实现了 Send 或 Sync
    // 反过来说只要有一个成员没有实现 Send 或 Sync，那么结构体也就不是 Send 或 Sync
    // 在 Rust 中大部分类型都实现了 Send 或 Sync，但是裸指针、Cell/RefCell、Rc 没有实现
    // 通常不需要手动实现 Send 或 Sync，因为需要非常小心的维护其安全性

    // 为裸指针实现 Send
    #[derive(Debug)]
    struct MySend(*mut u8);
    // 实现 Send trait，需要用 unsafe 标记
    // 注意实现了 Send trait 只能告诉 Rust 编译器可以认为此类型是安全的，
    // 但是实际的安全性需要自己保证，Rust 无法保证其结果的正确性
    unsafe impl Send for MySend {}
    let mut u = 5u8;
    let p = MySend(&mut u);
    let t = thread::spawn(move || {
        unsafe {
            *p.0 = 8;
        }
        println!("{:?} {}", p, unsafe { *p.0 });
    });

    t.join().unwrap();

    #[derive(Debug)]
    struct MySync(*const u8);
    unsafe impl Sync for MySync {}

    let ms: &'static MySync = &MySync(5 as *const u8);
    let t = thread::spawn(move || {
        println!("thread: {:?}", ms);
    });

    println!("main: {:?}", ms);

    t.join().unwrap();

}