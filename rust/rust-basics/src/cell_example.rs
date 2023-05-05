use std::{cell::{Cell, RefCell}, rc::Rc};


pub fn cell_example() {
    // Cell 只用于类型实现 Copy trait 的情况下才可以使用
    // &str 实现了 Copy trait
    let c = Cell::new("hello");
    // get 时由于类型实现了 Copy 特征，因此会拷贝出来一份新的，后续修改将不影响当前的变量
    let s1 = c.get();
    c.set("world");
    let s2 = c.get();
    println!("s1: {}, s2: {}", s1, s2);

    // 如果是所有权类型(没有实现 Copy)的类型在 Cell 中设置后之前获取的变量会失效
    let mut c = Cell::new(String::from("hello"));
    // 获取的是 Cell 底层的引用而不是 copy
    let s1 = c.get_mut();
    s1.push_str(" world");
    println!("s1: {}", s1);
    // set 后之前的引用会失效
    c.set(String::from("world"));
    let s2 = c.get_mut();
    // 之前的 s1 无法被引用
    // println!("s1: {}", s1);
    println!("s2: {}", s2);

    // Cell 不存在性能损耗并且不会出现 panic，因此如果类型实现 Copy 直接选择 Cell，否则再考虑 RefCell
}

pub fn refcell_example() {
    let s = RefCell::new(String::from("hello"));
    // 不可变引用
    // let s1 = s.borrow();
    // 可变引用
    // let s2 = s.borrow_mut();
    // 虽然编译不会报错但是运行时仍然会报错，即使使用 RefCell 也不能违反 Rust 中
    // 可变引用和不可变引用不能同时存在的规则，也就是说 RefCell 只是将借用规则从
    // 编译期间推迟到运行期间，而并不是绕过这个规则，因此违背规则会出现 panic
    // println!("s1: {}, s2: {}", s1, s2);

    // RefCell 最常用的场景就是编译时误报引用在多个地方使用，但实际上开发者能确信不违反
    // Rust 借用规则时，就可以使用 RefCell 绕开编译器的报错，从而直接运行，运行时只要不触发
    // 违背借用规则的条件，程序就不会 panic
    // RefCell 需要维护借用状态，因此会存在一点开销，因此只有在使用 Cell 无法满足需要时才使用 RefCell
    let mut s1: std::cell::RefMut<String> = s.borrow_mut();
    s1.push_str(" world");
    println!("s1: {}", s1);
    // 无法继续获取引用 所以可以先 drop 掉 s1
    drop(s1);
    // s2 和 s1 底层指向同样的数据
    let s2 = s.borrow();
    println!("s2: {}", s2);

    // 如果需要多个引用者 可以配合 Rc 来实现
    let s = Rc::new(RefCell::new(String::from("How")));
    let s1 = s.clone();
    let s2 = s.clone();
    s1.borrow_mut().push_str(" are");
    s2.borrow_mut().push_str(" you");
    // 所有引用底层都共享同一份数据，并且性能上也非常出色，内存占用上仅多出 3 个 usize/isize，
    // 64 位机器上也就是 24 个字节，并没有带来过多的开销
    println!("s: {}, s1: {}, s2: {}, rc num: {}", 
        s.as_ref().borrow().as_str(), s1.as_ref().borrow(), s2.as_ref().borrow(), Rc::strong_count(&s));

    // RefCell 的常用场景 在固定不可变引用的函数签名中实现数据修改
    let mq = MessageQueue{cache: RefCell::new(Vec::new())};
    mq.send("msg1".to_string());
    mq.send("msg2".to_string());
    let msgs = mq.cache.borrow();
    for msg in msgs.iter() {
        println!("{}", msg);
    }
}

// 假如存在第三方库定义的 trait 其中的方法定义是不允许修改的
pub trait Messager {
    fn send(&self, msg: String);
}

// 我们要基于 Messager 实现消息队列
struct MessageQueue {
    // 使用 RefCell 类型就可以通过不可变引用修改其内部数据
    cache: RefCell<Vec<String>>,
}

impl Messager for MessageQueue {
    fn send(&self, msg: String) {
        self.cache.borrow_mut().push(msg);
    }
}
