
/**
 * 基本数据类型 - 标量数据类型 
 * 标量（scalar）类型表示一个单独的值，标量类型有 4 种：整型、浮点型、布尔类型和字符类型
 * 标量类型存储在栈中，每次拷贝都是拷贝整个值
 */
pub fn data_type_basic() {
    // let 定义变量是不可变的
    // 自动推导类型 i32 
    let a = 10;
    // 手动指定类型
    let b: i32 = 20;
    // 字面量类型注解
    let c = 30i32;
    // 或者是加 _ 区分
    let d = 30_i32;

    // 调用函数返回
    let e = add(add(a, b), add(c, d));
    let e1 = add1(&add1(&a, &b), &add1(&c, &d));

    println!("(a + b) + (c + d) = {} = {}", e, e1);

    // 通过 let mut 定义可变变量
    let mut ma = 6;
    println!("mut ma is {ma}");
    // mut 类型的变量前后赋值类型一定相同，因为变量本身的内存没有改变
    ma = 8;
    println!("mut ma is {ma}");

    // 变量遮蔽 shadowing
    let yq = 5;
    // 允许定义相同的变量名称覆盖之前的变量，类型并没有限制
    let yq = yq + 1;

    {
        // 进入内部作用域
        // 变量支持 shadowing （遮蔽或隐藏），相同的变量会在当前作用域内覆盖以前的变量
        // 可以重复用相同的变量名定义，而且变量类型没有限制，因为完全是一个新的变量
        // 变量多次定义都相当于新的对象，会多次分配内存，而 mut 只会分配1次
        let yq = yq * 2;
        println!("yq is value: {}", yq);
    }

    // 遮蔽的变量生命周期结束，恢复外部作用域的值
    println!("yq is value: {}", yq);

    // 常量定义，使用 const 关键字，必须声明类型
    // 常量可以在任何作用域中定义，可以被其他多个部分的代码共享
    // 常量在编译时进行嵌入，因此在程序的整个生命周期内都生效
    const SECONDS: u32 = 5 * 60;
    println!("const SECONDS: {SECONDS}");

    // 通过前导下划线表示让编译器忽略不使用的变量而不警告
    let _x = 3;

    // 下划线可以方便表示比较长的数字，提高可读性，不影响数值本身
    let f: i64 = 1_000_000;
    println!("{}", f);

    // 默认写法是十进制表示，也可以用十六进制、八进制、二进制等字面量的方式表示
    let h = 0xff;
    let o = 0o77;
    let bin = 0b1111_0000;
    // 输出默认都是十进制方式
    println!("h: {h}, o: {o}, bin: {bin}");

    // 也可以分别用不同的占位符表示不同的输出
    println!("{:b}, {:o}, {:x}", bin, o, h);

    // 有符号类型i8, i16, i32, i64
    let v1: i8 = 68;
    let v2: i16 = 10467;
    let v3: i32 = 2889;
    let v4: i64 = 65566;
    println!("v1: {}, v2: {}, v3: {}, v4: {}", v1, v2, v3, v4);

    // 无符号类型u8, u16, u32, u64
    let v5: u8 = 252;
    let v6: u16 = 65535;
    let v7: u32 = 0;
    let v8: u64 = 66666;
    println!("v5: {}, v6: {}, v7: {}, v8: {}", v5, v6, v7, v8);

    // 与CPU位宽一致的类型 isize,usize
    // 在 32 位 CPU 上等同于 i32 以及 u32，在 64 位 CPU 上等同于 i64 以及 u64
    let v9: isize = -102;
    let v10: usize = 201;
    println!("v9: {}, v10: {}", v9, v10);

    // 浮点数 f32 f64，自动推导是 f64
    let v11: f32 = 3.1415;
    let v12: f64 = 2.71828;
    println!("v11: {}, v12: {}", v11, v12);

    // 数字都支持各类运算，包括：四则运算、取余等，整数除法和其他语言一样会舍入
    // 加法
    let sum = 5 + 10;

    // 减法
    let difference = 95.5 - 4.3;

    // 乘法
    let product = 4 * 30;

    // 除法
    let quotient = 56.7 / 32.2;
    // 整数相除向下舍入
    let truncated = -5 / 3; // 结果为 -1

    // 取余
    let remainder = 43 % 5;

    println!("sum: {sum}, difference: {difference}, \
             product: {product}, quotient: {quotient}, \
             truncated: {truncated}, remainder: {remainder}");

    // 布尔类型 只有 true 和 false 主要用在控制语句中
    let t = true;
    let f: bool = false;
    println!("true: {t}, false: {f}");

    // 字符类型 char，字面量必须用单引号包裹，双引号是字符串
    // 每个字符占 4 个字节
    let c = 'z';
    let z: char = '中';
    let heart_eyed_cat = '😻';
    println!("c: {c}, z: {z}, emoji: {heart_eyed_cat}");
    // 单字节字符字面写法，本质是 u8 类型，前面必须带 b
    // 只能写 ASCII 字符，不能超过 1 个字节
    let vb = b'A';
    // 输出是数字
    println!("byte: {vb}");

}

// 定义加法函数
// 函数参数（形参）和返回值都必须强类型指定
fn add(i: i32, j: i32) -> i32 {
    i + j
}

// 函数带显式生命周期
fn add1<'a, 'b>(i: &'a i32, j: &'b i32) -> i32 {
    *i + *j
}
