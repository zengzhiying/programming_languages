pub fn vector_type_example() {
    // 创建动态数组 必须指定类型
    let _v1: Vec<i32> = Vec::new();
    // 可变数组如果放入元素, 则无需指定类型, 会自动推断
    let mut v1 = Vec::new();
    // 尾部插入元素
    v1.push(3);

    // 创建时指定大小, 提升性能
    let mut v1 = Vec::with_capacity(10);
    v1.push(3);

    println!("{:?}", v1);

    // 使用宏直接创建 vector, 会自动推断类型
    let v2 = vec![1, 2, 3];
    println!("{:?}", v2);

    // vector 和其他对象完全一样, 超出作用域会自动删除
    // 下标索引访问元素, 注意不要越界
    let n = &v2[2];
    println!("v2[2]: {}", n);
    // get 方式访问, 返回 Option, 越界会返回 None
    match v2.get(2) {
        Some(n) => println!("v2[2]: {}", n),
        None => println!("v2[2] not exists.")
    }

}