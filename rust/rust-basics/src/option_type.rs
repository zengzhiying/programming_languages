// Option 是标准库中的枚举类型，应用非常广泛
// 简单来说 Option 枚举的场景是一个值要么有值要么没值
// 在其他语言中经常有 null 值，如果编码时不小心很容易出现空指针的错误
// 但是 Rust 中没有直接的 null 值，但是空值的概念是有意义的，因为外部设备是不可靠的，由于故障会经常出现缺失值
// 所以在 Rust 中使用的是 Option 枚举来编码存在或者不存在的值，这样就可以避免其他语言中常见的 BUG
// Option 枚举在标准库的定义如下：
// enum Option<T> {
//     None,
//     Some(T),
// }
// 由于 Option 用的非常多，因此直接包含在 prelude 之中，所以我们不需要显示引入作用域
// 使用其中的成员也不需要 Option:: 而是直接可以使用，但是我们要明白 Some(T) 和 None 并不是独立的类型，而是 Option<T> 的成员
// 使用 Option<T> 无法直接参与到类型 T 的计算中，所以在使用时必须进行显式的转换，这样的话 Rust 在编译器级别就可以进行检查
// 从而避免空值的情况发生，因为语法要求我们必须强制进行判断，无论是 Some(T) 还是 None 都要在我们的控制流中进行处理

pub fn option_type() {
    // 如果使用 Some 指定值，则会自动推断
    let f = Some(5);
    // 如果是 None 值必须显式指定类型
    let p: Option<i32> = None;
    match_option(f);
    match_option(p);
    println!("{:?}", f);
}

fn match_option(x: Option<i32>) {
    // 使用 match 匹配
    match x {
        None => println!("x is None."),
        // 注意变量名称如果和之前的 x 一致, 则会出现变量覆盖, 一直到作用域结束
        // 这点要注意, 尽量使用不同的变量名
        Some(i) => println!("x is {}", i)
    }
}