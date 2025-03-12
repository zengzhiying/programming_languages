
/**
 * 复合（Compound）类型可以将多个值组合成一个类型
 * 当前示例包含两类：元组（tuple）和数组（array）
 * 元组和数组都存储在栈上，赋值方式为拷贝
 * 另外结构体、枚举、Vec/Map 等其实也属于复合类型，会在另外的示例中单独列出
 */
pub fn compound_data_type() {
    // 复合类型 - 元组（tuple）
    // 元组长度固定，一旦声明，其长度不会变化
    // 元组每个位置的元素类型也不必相同，可以单独定义
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    // 访问时可以用相同数量的变量解构
    let (x, y, z) = tup;
    println!("The {:?} value of x: {x} y: {y} z: {z}", tup);

    // 解构时可以引入可变变量
    let (b1, mut b2) = (true, false);
    println!("b1 = {:?}, b2 = {:?}", b1, b2);
    b2 = true;
    assert_eq!(b1, b2);

    // 也可以用索引来访问
    let x = tup.0;
    let z = tup.2;
    println!("The {:?} value of x: {x} z: {z}", tup);

    // 解构式赋值
    let (x1, x2, x3, x4, x5);
    // 赋值时自动推断类型
    (x1, x2) = (1, 2);
    // 按照顺序匹配 后面的_用来占位 所以分别为2,3,6
    [x3, x4, .., x5, _] = [2, 3, 4, 5, 6, 7];
    assert_eq!([x1, x2, x3, x4, x5], [1,2,2,3,6]);

    // 不带任何值的元组也叫做单元（unit）类型，表示空值或空类型
    // 如果表达式或函数不返回任何值，则会隐式返回单元值
    let un = ();
    println!("unit value: {:?}", un);

    // 复合类型 - 数组（array）
    // 数组中所有元素的类型必须一致，而且长度也是固定不可变的
    // 当确定元素个数不会改变时，数组会非常有用
    // 定义时可以根据上下文自动推断类型，类型包括元素类型和长度，比如：[f32, 3]
    let numbers = [
        32.0,
        16.5f32,
        89.1_f32,
    ];
    // 使用索引访问数组，Rust 运行时会进行越界检查，如果访问越界会直接 panic 
    println!("{:02}", numbers[0]);

    // 创建每个元素都为相同值的数组，下面每个值都为 3，长度是 5
    let numbers = [3; 5];
    println!("numbers: {:?}", numbers);

    // 二维数组写法，下面的二维数组在编译时会直接影响前面的类型推断
    let a = [1,2,3];
    let b: [u8; 3] = [1,2,3];
    let c = [0; 3];
    let d: [u8; 3] = [0; 3];
    // 其中每个数组类型必须一致
    let arrs = [a, b, c, d];

    // 数组迭代
    for arr in arrs {
        print!("{:?}: ", arr);
        for n in arr.iter() {
            print!("\t {}", n);
        }

        let mut sum = 0;
        for i in 0..arr.len() {
            sum += arr[i];
        }

        println!("\t ∑{:?} = {}", arr, sum);
    }


    // 数字比较示例
    // 同类型的数字可以直接比较
    let e1: i32 = 761;
    let e2: i32 = 661;
    if e1 >= e2 {
        println!("e1 >= e2.")
    }

    // 不同类型比较需要转换，注意降级转换的精度损失
    let e3: i16 = 671;
    if e2 <= e3 as i32 {
        println!("e2 <= e3");
    }

    // 新版本不需要再导入 use std::convert::TryInto; 包了
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
}