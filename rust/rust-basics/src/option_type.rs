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
// 由于 Option 用的非常多，因此直接包含在 prelude 之中，所以我们不需要显式引入作用域
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
    let f1 = plus_one(f);
    println!("{:?}", f1);
    // 除 match 外三种不同的解包方式
    // unwrap，如果值为 None 会直接 panic
    assert_eq!(f1.unwrap(), 6);
    // expect，如果值为 None 也会 panic，但是会携带指定的提示信息
    assert_eq!(f1.expect("f1 value is None"), 6);
    // unwrap_or，如果为 None 将由默认参数替代
    assert_eq!(f1.unwrap_or(0), 6);

    // Option 的所有权跟随类型 T，如果 T 是所有权类型那么 Option 的行为一致
    // 如果 T 实现了 Copy trait，那么 Option 也可以拷贝
    let v = Some(String::from("abc"));
    assert_eq!(v.unwrap(), "abc");

    // 通过 map 处理 Option 避免捷豹
    // 如果值正常则会应用 map，如果值为 None 则结果仍然是 None
    let v = Some(String::from("hello"));
    // map 会消耗所有权
    let vl = v.map(|s| s.len());
    assert_eq!(vl, Some(5));

    let s = String::from("string value");
    let sv = Some(&s);
    // cloned 要求 T 必须实现 Clone trait，这样 sv1 相当于创建新的所有权变量，而不影响之前的
    let sv1 = sv.cloned();
    assert_eq!(sv1, Some(s));

    // is_some 和 is_none 快捷判断是否有值
    assert!(sv1.is_some());
    let none: Option<i32> = None;
    assert!(none.is_none());
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

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1)
    }
}

#[cfg(test)]
mod tests {
    // 父模块的内部方法同样可以被访问到
    use super::*;

    #[test]
    fn test_plus_one() {
        assert_eq!(plus_one(Some(5)), Some(5 + 1));
    }
}
