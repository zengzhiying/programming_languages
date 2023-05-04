use std::cell::Cell;


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
