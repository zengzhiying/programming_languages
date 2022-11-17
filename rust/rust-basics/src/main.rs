mod basic_type;
mod complex_type;
mod control;
mod generics_type;
mod array_slice_type;
mod structure_type;

fn main() {
    println!("Hello, Rust!");

    string_basic();
    text_process();

    basic_type::data_type_basic();

    complex_type::complex_type();

    control::for_control();
    // control::while_control();
    control::break_control();
    control::if_else_control();
    control::match_control();

    generics_type::generics_add();

    array_slice_type::array_type();
    let mut buf = structure_type::new_buffer(String::from("hello"), Vec::new());
    structure_type::print_buffer(&mut buf);
    println!("buf: {:?}", buf);
}

fn string_basic() {
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
}

fn text_process() {
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
