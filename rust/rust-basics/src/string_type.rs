use std::ops::Add;

pub fn string_basic() {
    // 字符串字面量值
    let a = "Hello Rust.";
    let b = "你好, Rust.";

    // 字符串数组数组
    let ss = [a, b];

    for s in ss.iter() {
        // &s表示借值 只读访问
        println!("{}", &s);
    }

    let heap_str = str_to_string(a);
    println!("String: {}", heap_str);


    // 字符串相加
    let sa: String = String::from("hello ");
    let sb1: String = String::from("ru");
    let sb2: String = String::from("st");
    // 字符串后面需要使用切片累加, 底层是调用 add 方法
    // 注意累加完成 sa 的所有权会转到 sc
    let sc = sa + &sb1 + &sb2;
    println!("sc: {}", sc);
    // 转为可变并且累加
    let mut sc = sc;
    sc += "!";
    println!("sc: {}", sc);

    // 使用 add 方法 一定要用返回值接受所有权
    let sc = sc.add("!!");
    println!("sc: {}", sc);

    // 使用 format 格式化字符串
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

fn str_to_string(s: &str) -> String {
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