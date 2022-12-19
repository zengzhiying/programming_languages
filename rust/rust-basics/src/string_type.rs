use std::ops::Add;

pub fn string_basic() {
    // 字符串
    let a = "Hello Rust.";
    let b = "你好, Rust.";

    // 数组
    let ss = [a, b];

    for s in ss.iter() {
        // &s表示借值 只读访问
        println!("{}", &s)
    }

    let s1 = &a[..2];
    let s2 = &a[3..];
    println!("s1: {}, s2: {}", s1, s2);

    // 切分UTF-8字符必须保证单元性
    let s1 = &b[..3];
    println!("s1: {}", s1);

    let s3 = str_to_string(a);
    println!("s3: {}", s3);


    // 字符串相加
    let sa: String = String::from("hello ");
    let sb1: String = String::from("ru");
    let sb2: String = String::from("st");
    // 字符串后面需要使用切片
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