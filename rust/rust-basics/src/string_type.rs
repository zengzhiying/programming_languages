use std::ops::Add;

// 字符串无论是 &str 还是堆上的 String，都必须是 UTF-8 编码的，不支持二进制
// String 底层其实是 Vec<u8> 的封装，但是会确保必须是有效的 UTF-8 编码
// 字符串不支持下标索引直接取值，但是可以切片，不支持的原因是下标取值从语义上来说复杂度是 O(1)，
// 但是 Rust 必须保证字符串是合法的 UTF-8 编码，所以使用下标取值时就必须从开头进行有效字符的遍历，
// 这破坏了语义从而无法保证性能，因此编译器不允许字符串通过下标取值，这部分的限制是严格的，可以避免字符串的复杂性给我们造成的困扰
// 虽然有上面的限制，但是 Rust 标准库中仍然提供了比如 contains 和 replace 等方法来帮助我们解决复杂性
// crate 上也有一些相关的开源模块可以使用
pub fn string_basic() {
    // 字符串字面量值
    // 存储在程序二进制中，所有的字符串字面量类型都是 &str 字符串切片
    let a = "Hello Rust.";
    let b = "你好, Rust.";

    // 字符串切片数组
    let ss = [a, b];

    for s in ss.iter() {
        // &s表示借值 只读访问
        println!("{}", &s);
    }

    // 创建空的 String 符合所有权规则
    let mut s = String::new();
    // 放入数据，必须是可变类型
    s.push('1');
    s.push_str("23");
    println!("{}", s);

    // &str 转 String
    let heap_str = str_to_string(a);
    println!("String: {}", heap_str);

    // 通过 from 从 &str 创建 String
    let _s = String::from("contents");

    // 字符串相加
    let sa: String = String::from("hello ");
    let sb1: String = String::from("ru");
    let sb2: String = String::from("st");
    // 字符串后面需要使用切片累加, 底层是调用 add 方法，原型是：add(self, s: &str) -> String 所以 self 会发生所有权转移
    // 注意累加完成 sa 的所有权会转到 sc，sa 无法再使用，但是 sb1 和 sb2 是 &String 方式传参，因此可以继续使用
    // 在调用方法是会将 &String 强转为 &str，这个过程是编译器自动完成的，称作 Deref 强转：&String -> &String[..] -> &str
    // sa.add(&sb1).add(&sb2);
    let sc = sa + &sb1 + &sb2;
    println!("sc: {}", sc);
    // 转为可变并且累加
    let mut sc = sc;
    sc += "!";
    println!("sc: {}", sc);

    // 使用 add 方法 一定要用返回值接受所有权
    let sc = sc.add("!!");
    println!("sc: {}", sc);

    // 使用 format! 宏格式化字符串
    // format! 生成的代码使用引用，因此不会有任何所有权的转移
    let f = format!("{} {}", String::from("hello"), "happy!");
    println!("format: {}", f);

    // 原始 ASCII 字符写法
    let bs = "write: \x52\x76\x6f.";
    println!("bs: {}", bs);
    // unicode 字符写法
    let un = "u: \u{211d}";
    println!("un: {}", un);

    // UTF-8 字符串遍历
    let s = "This is: 华夏";
    // 字符遍历
    for c in s.chars() {
        print!("{}", c);
    }
    println!();
    // 字节遍历
    for b in s.bytes() {
        print!("{} ", b);
    }
    println!();
}

// &str 切片转 String
fn str_to_string(s: &str) -> String {
    // to_string 方法能用于任何实现了 Display trait 的实例
    // 相当于创建一个新的对象，然后将所有权返回到外部
    s.to_string()
}

pub fn text_process() {
    // 多行文本
    let lines = "
    name, len
    Java,6
    Python,8
    Rust,10
    PHP,2
    Go,3
    Invalid,err
    Scala,8
    ";
    let records = lines.lines();
    for (i, row) in records.enumerate() {
        if i == 0 || row.trim().len() == 0 {
            continue;
        }

        let columns : Vec<_> = row
            .split(",")
            .map(|f| f.trim())
            .collect();

        // debug调试
        if cfg!(debug_assertions) {
            eprintln!("debug: {:?} -> {:?}", 
                row, columns)
        }

        if columns.len() != 2 {
            continue;
        }

        let name = columns[0];

        if let Ok(len) = columns[1].parse::<f32>() {
            println!("name: {}, len: {}", name, len);
        }
        
    }
}