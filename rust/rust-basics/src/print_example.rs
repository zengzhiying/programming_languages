
pub fn print_example() {
    let s = "hello";
    // 不带换行符
    print!("print {} ", s);
    // 带换行符
    println!("println {}", s);
    // 格式化返回字符串 String 类型
    let s1 = format!("{} rust", s);
    println!("println s1: {}", s1);

    // 标准错误
    let msg = "error msg";
    eprint!("eprint {} ", msg);
    eprintln!("eprintln {}", msg);

    // 位置参数
    println!("nums: {}, {}", 1, 2);
    println!("nums: {1}, {0}", 1, 2);
    println!("nums: {}, {1}, {}", 1, 2);

    // 具名参数 必须放在不具名参数之后
    println!("args: {arg1}, {arg2}", arg1="-f", arg2="-t");
    println!("{name} id {}", 2, name="bob");

    // 浮点数保留小数点位数
    println!("f1: {:.2}, f2: {:.3?}", 3.141592, 2.71828);

    // 补齐宽度 数字是右填充 字符串是左填充
    println!("{:03} {:5}!", 99, "yes");

    // 进制表示
    println!("二进制: {:#b}", 96);
    println!("八进制: {:#o}", 96);
    println!("小写十六进制: {:#x}", 127);
    println!("大写十六进制: {:#X}", 127);
    println!("无前缀的十六进制: {:x}", 127);
    // 0 填充，宽度为 10
    println!("使用 0 填充二进制: {:#010b}", 96);

    // 科学计数法表示
    println!("{:2e}", 1000000000);
    println!("{:2E}", 1000000000);

    // 指针地址
    let v = vec![1, 2, 3];
    // &v 是栈上地址 v.as_ptr() 是堆上地址 两者相差比较大
    // v.as_ptr() = &v[0]
    println!("v: {:?} addr: {:p} {:p} {:p}", v, &v, v.as_ptr(), &v[0]);
    let v = [1, 2, 3];
    // &v 和 v.as_ptr() 都在栈上 地址相同
    println!("v: {:?} addr: {:p} {:p} {:p}", v, &v, v.as_ptr(), &v[0]);
}
