// 结构体中的属性使用引用时, 也需要生命周期标注
struct Partition<'a> {
    part: &'a str
}


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

    let partition = Partition{
        part: "202102_2_1"
    };
    println!("partition: {}", partition.part);

    let mut partition = Partition{part: ""};
    {
        // 不能使用字符串切片的方式
        // let h = String::from("hello");
        // let h = h.as_str();
        // 可以直接使用 &str 字面量
        let h = "hello";
        partition.part = h;
    }
    println!("partition: {}", partition.part);

    // let partition;
    // {
    //     let part = String::from("hello");
    //     let part = part.as_str();
    //     println!("part: {}", part);
    //     partition = Partition{part: part};
    // }
    // 字符串切片生命周期小于结构体实例的生命周期 如果打印也会报错
    // println!("partition: {}", partition.part);
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


