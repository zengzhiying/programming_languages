use std::fmt::Display;
// 结构体中的属性使用引用时, 需要生命周期标注
struct Partition<'a> {
    part: &'a str
}

// 实现方法 注意此时泛型生命周期本身已经成为方法的一部分
impl<'a, 'b> Partition<'a> {
    // 这种情况下不需要标注生命周期
    // 编译器会自动消除
    fn get_part(&'a self, _: &'b str) -> &'b str
    where 'a: 'b {
        self.part
    }
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
    println!("partition get_part: {}", partition.get_part(""));

    // let partition;
    // {
    //     let part = String::from("hello");
    //     let part = part.as_str();
    //     println!("part: {}", part);
    //     partition = Partition{part: part};
    // }
    // 字符串切片生命周期小于结构体实例的生命周期 如果打印也会报错
    // println!("partition: {}", partition.part);


    let string1 = String::from("abcd");
    let string2 = "xyz";

    let result = longest_with_an_announcement(
        string1.as_str(),
        string2,
        "Today is someone's birthday!",
    );
    println!("The longest string is {}", result);
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

// 泛型和生命周期同时存在的情况，
// 因为生命周期也属于泛型，所以两者是写到同一个 <> 里面的
fn longest_with_an_announcement<'a, T>(
    x: &'a str,
    y: &'a str,
    ann: T,
) -> &'a str
where
    T: Display,
{
    println!("Announcement! {}", ann);
    if x.len() > y.len() {
        x
    } else {
        y
    }
}


