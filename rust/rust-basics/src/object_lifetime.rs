pub fn lifetime_example() {
    let x = "hello";
    let y = "rust";
    let z = longest(x, y);
    println!("longest str: {}", z);

    let s1 = "rust";
    {
        let s2 = "xyz";
        let r = longest(s1, s2);
        println!("longest str: {}", r);
    }
}

// 使用生命周期标注, 使得变量 x 和 y 的生命周期至少是 a，
// 这样编译器就不会因为生命周期不确定而报错了
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
