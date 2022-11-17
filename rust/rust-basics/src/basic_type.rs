// 导入类型转换的包
use std::convert::TryInto;

// 基本数据类型
pub fn data_type_basic() {
    // 自动推导 i32 let定义变量是只读的
    let a = 10; 
    // 定义类型
    let b: i32 = 20;
    // 字面量类型注解
    let c = 30i32;
    // 或者是
    let d = 30_i32;

    let e = add(add(a, b), add(c, d));
    let e1 = add1(&add1(&a, &b), &add1(&c, &d));

    println!("(a + b) + (c + d) = {} = {}", e, e1);

    // 忽略变量不警告
    let _x = 3;

    // 下划线可以方便表示 不影响数字
    let f: i64 = 1_000_000;
    println!("{}", f);
    // 数组 元素类型必须一致
    let numbers = [
        // 根据上下文自动推断类型
        32.0,
        16.5f32,
        89.1_f32,
    ];
    println!("{:02}", numbers[0]);

    // 二进制字面量
    let t = 0b111;
    // 八进制字面量
    let o = 0o36;
    // 16进制字面量
    let x = 0x1f;
    // 格式化默认为10进制, 分别用不同的占位符表示不同的输出
    println!("{}, {:b}, {:o}, {:x}", t, x, t, o);

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

    // 浮点数f32 f64
    let v9: f32 = 3.1415;
    let v10: f64 = 2.71828;
    println!("v9: {}, v10: {}", v9, v10);

    // 与CPU位宽一致的类型 isize,usize
    // 在64位CPU上等同于i64以及u64
    let v11: isize = -102;
    let v12: usize = 201;
    println!("v11: {}, v12: {}", v11, v12);

    // 同类型的数字可以直接比较
    let e1: i32 = 761;
    let e2: i32 = 661;
    if e1 >= e2 {
        println!("e1 >= e2.")
    }

    // 不同类型比较需要转换 注意降级转换的精度损失
    let e3: i16 = 671;
    if e2 <= e3 as i32 {
        println!("e2 <= e3");
    }

    let e3_ = e3.try_into().unwrap();
    if e2 <= e3_ {
        println!("e2 <= e3_");
    }

    // 浮点数判断需要谨慎
    // 定义浮点数元组
    let abc: (f32, f32, f32) = (0.1, 0.2, 0.3);
    let xyz: (f64, f64, f64) = (0.1, 0.2, 0.3);

    // 查看底层的存储内容
    println!("f32 0.1 + 0.2 = {:x}", (abc.0 + abc.1).to_bits());
    println!("f32 0.3: {:x}", abc.2.to_bits());

    println!("f64 0.1 + 0.2 = {:x}", (xyz.0 + xyz.1).to_bits());
    println!("f64 0.3: {:x}", xyz.2.to_bits());

    assert!(abc.0 + abc.1 == abc.2);  // 成功
    // assert!(xyz.0 + xyz.1 == xyz.2);  // 崩溃
    // 建议使用范围比较 成功
    assert!((xyz.2 - xyz.0 - xyz.1).abs() <= f64::EPSILON);

    // 浮点数可能会有无穷大或NAN的结果 可以使用is_nan或者is_infinite来判断
    let x_ = (-4.0f32).sqrt();
    if x_.is_nan() {
        println!("x_ is NAN");
    }

    let y_: f32 = 1.0 / 0.0;
    // is_infinite等效于!is_finite
    if y_.is_infinite() {
        println!("y_ is infinite.")
    }

    // 变量解构
    let (b1, mut b2) = (true, false);
    println!("b1 = {:?}, b2 = {:?}", b1, b2);
    
    b2 = true;

    assert_eq!(b1, b2);

    // 解构式赋值
    let (x1, x2, x3, x4, x5);
    (x1, x2) = (1, 2);
    // 按照顺序匹配 后面的_用来占位 所以分别为2,3,6
    [x3, x4, .., x5, _] = [2, 3, 4, 5, 6, 7];

    assert_eq!([x1, x2, x3, x4, x5], [1,2,2,3,6]);


    // 常量
    const MAX_VALUE: u32 = 3;
    println!("const value: {}", MAX_VALUE);

    // 变量遮蔽 shadowing
    // 变量多次绑定相当于新的对象 会多次分配内存 而mut只会分配1次
    let yq = 5;
    let yq = yq + 1;

    {
        // 内部作用域
        let yq = yq * 2;
        println!("yq is value: {}", yq);
    }

    // 外部值
    println!("yq is value: {}", yq);



}

// 定义加法函数
fn add(i: i32, j: i32) -> i32 {
    i + j
}

// 函数带显式生命周期
fn add1<'a, 'b>(i: &'a i32, j: &'b i32) -> i32 {
    *i + *j
}
