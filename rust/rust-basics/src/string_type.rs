use std::ops::Add;

/**
 * 在 Rust 中字符串无论是 &str 还是堆上的 String，都必须是 UTF-8 编码的，不支持二进制
 * String 底层其实是 Vec<u8> 的封装，但是会确保必须是有效的 UTF-8 编码
 * 
 * 字符串不支持下标索引直接取值，但是可以切片，不支持的原因是下标取值从语义上来说复杂度是 O(1)，
 * 但是 Rust 必须保证字符串是合法的 UTF-8 编码，所以使用下标取值时就必须从开头进行有效字符的遍历，
 * 这破坏了语义从而无法保证性能，因此编译器不允许字符串通过下标取值，这部分的限制是严格的，可以避免字符串的复杂性给我们造成的困扰
 * 虽然有上面的限制，但是 Rust 标准库中仍然提供了比如 contains 和 replace 等方法来帮助我们解决复杂性，crate 上也有一些相关的开源模块可以使用
 */
pub fn string_basic() {
    // 字符串字面量值
    // 存储在程序二进制中，所有的字符串字面量类型都是 &str 字符串切片
    let a = "Hello Rust.";
    let b = "你好, Rust.";

    // 字符串切片数组
    let ss = [a, b];

    for s in ss.iter() {
        // 这里无论是使用 s 或者 &s 访问都是可以的
        // 编译器会识别出底层要访问的就是字符串
        println!("{}", &s);
    }

    // 可以使用 \x 表示原始的 ASCII 字符，必须是可显示的 ASCII 范围，超过了会报错
    // Rust 编译器会根据 \x 指定的字符逐个字面解释，不会放到一块解码
    let c = "Very goo\x64\x21";
    println!("{}", c);
    let bs = "write: \x52\x76\x6f.";
    println!("bs: {}", bs);
    // 还可以使用 \u 直接写入 Unicode 字符，Rust 编译器会自动编码为 UTF-8
    let d = "大学之\u{9053}";
    println!("{}", d);
    let un = "u: \u{211d}";
    println!("un: {}", un);

    // 原始字面量写法，Rust 不进行任何转义
    // 如果字符串中恰好有 "# 之类的闭合符号，那么只需要开头结尾多加一个 #，让 Rust 知道匹配开始和结束即可
    let raw_str = r##"\x38 \u{1001} "# \n \t \
    \"
    \n"##;
    println!("{}", raw_str);

    // 创建空的 String 符合所有权规则
    let mut s = String::new();
    // 放入数据，前提字符串定义时必须是可变类型
    s.push('1');
    s.push_str("23");
    println!("{}", s);

    // &str 转 String
    let heap_str = str_to_string(a);
    println!("String: {}", heap_str);

    // String 转 &str，下面两种方法等效，&str 都是 String 的切片引用，底层数据不变
    let heap_str_slice1 = &heap_str[..];
    let heap_str_slice2 = heap_str.as_str();
    
    println!("slice1: {}, slice2: {}", heap_str_slice1, heap_str_slice2);

    // 转成字节数组，就是相当于底层 Vec<u8> 的 as_slice
    let heap_str_u8 = heap_str.as_bytes();
    assert_eq!(vec![72u8,101,108,108,111,32,82,117,115,116,46].as_slice(), heap_str_u8);


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

    // 使用 add 方法 通过函数返回值返回值接收所有权
    let sc = sc.add("!!");
    println!("sc: {}", sc);

    // 使用 format! 宏格式化字符串
    // format! 生成的代码使用引用，因此不会有任何所有权的转移
    let f = format!("{} {}", String::from("hello"), "happy!");
    println!("format: {}", f);

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

    // Vec<u8> 或字节切片 &[u8] 也可以直接转换回字符串
    let bs: Vec<u8> = vec![84, 104, 105, 115, 32, 105, 115, 58, 32, 229, 141, 142, 229, 164, 143];
    let bs_string = String::from_utf8(bs);
    // str 同样有对应的方法将 &[u8] 转为 &str
    // use std::str as stdstr;
    // stdstr::from_utf8(&bs);
    // 转换不一定成功，需要进行异常处理
    match bs_string {
        Ok(s) => println!("bs string: {}", s),
        Err(err) => println!("Vec<u8> -> string err: {}", err)
    }

    // 还可以使用 unsafe 方法 from_utf8_unchecked，这个方法不会进行任何校验，返回的结果可能会出现未知的乱码情况
    let _bs_string1 = unsafe {
        String::from_utf8_unchecked(vec![84, 104, 105, 115, 32, 105, 115, 58, 32, 229, 141, 142, 229, 164, 143])
    };
    
    // 字符串还具备比较强大的 parse 方法，可以将字符串转换成其他数据类型
    let str_num = "10".parse::<u32>();
    // 正常需要处理错误
    println!("str to num: {}", str_num.unwrap());
    // 还可以使用自动推断的方法
    let _str_num: u32 = "10".parse().unwrap();
    let str_float = "4.2".parse::<f32>().unwrap();
    println!("str to float: {}", str_float);
    let str_bool = "true".parse::<bool>().unwrap();
    println!("str to bool: {}", str_bool);
    let str_char = "道".parse::<char>().unwrap();
    println!("str to char: {}", str_char);
    let str_ip_addr = "10.10.1.21".parse::<std::net::Ipv4Addr>().unwrap();
    println!("str to ip: {}", str_ip_addr);

}

// &str 切片转 String
fn str_to_string(s: &str) -> String {
    // to_string 方法能用于任何实现了 Display trait 的实例
    // 相当于创建一个新的对象，然后将所有权返回到外部
    s.to_string()
}

/**
 * 文本字符串处理示例代码
 */
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