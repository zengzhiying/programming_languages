#[derive(Debug)]
struct SubDrop1;
#[derive(Debug)]
struct SubDrop2;

impl Drop for SubDrop1 {
    fn drop(&mut self) {
        println!("Dropping SubDrop1");
    }
}
impl Drop for SubDrop2 {
    fn drop(&mut self) {
        println!("Dropping SubDrop2");
    }
}

#[derive(Debug)]
struct ParentDrop {
    sub1: SubDrop1,
    sub2: SubDrop2
}

impl Drop for ParentDrop {
    fn drop(&mut self) {
        println!("Dropping ParentDrop");
    }
}

#[derive(Debug)]
struct Foo;
impl Drop for Foo {
    fn drop(&mut self) {
        println!("Dropping Foo")
    }
}

pub fn drop_example() {
    let v = ParentDrop{
        sub1: SubDrop1{},
        sub2: SubDrop2{}
    };

    let _f = Foo;

    println!("running: {:?} sub1: {:?} sub2: {:?}", v, v.sub1, v.sub2);
    // 通过实现 drop 方法可以查看回收资源的顺序
    // 变量级别：按照栈的顺序，后初始化的变量先被回收
    // 结构体级别：结构体存在嵌套的或者内部变量的情况，从外到内依次释放

    // 无论是否实现 drop，在离开作用域时都会调用内部实现的 drop 来释放资源
    // drop 并不会发送所有权转移，因为 drop 方法使用的是可变引用，但是 drop 方法不能直接调用
    // 而是通过 drop() 发送所有权转移并释放掉内存，所以可以保证所有权变化是和 drop 是同时发生，从而避免垂悬引用
    let f = Foo;
    // 不允许调用 f.drop
    // f.drop();
    // 只能调用 drop 函数，这个函数会发生所有权转移，后面再使用变量时编译器肯定不会通过
    drop(f);
    // println!("dropped: {:?}", f);
    println!("dropped f");
    // 我们不需要手动去调用 drop 释放资源，因为 Rust 编译器会自动帮我们完成这些工作
    // 但是一些文件描述符、socket 等资源则需要手动释放，可以在代码中提前释放，也可以实现 drop 方法让 Rust 自动调用帮我们释放

    // 另外无法为类型同时实现 Copy 和 Drop trait，因为实现了 Copy 的类型会被编译器隐式复制，所以没有明确的析构时间
}