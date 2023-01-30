
pub fn option_type() {
    let f = Some(5);
    // None 值必须显式指定类型
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