
pub fn slice_type() {
    // 字符串切片
    // 切片是指向原来字符串数据的引用，切片可以理解为只读引用的一种形式
    // 字符串字面值本身也是一个切片，所以属于只读引用，当然也就是不可变的
    let s = "Hello Rust!";
    let s1 = &s[..2];
    let s2 = &s[3..];
    println!("s1: {}, s2: {}", s1, s2);

    let s = "你好, Rust!";
    // 切分 UTF-8 字符串必须保证字符的单元性，否则运行时会报错
    let s1 = &s[..3];
    println!("s1: {}", s1);

    // 堆上的字符串切分方法一致
    // 在使用 &String 的场景最好采用 &str 更加符合 Rust 规范
    let mut s = String::from("hello Rust!");
    let s1 = &s[..2];
    let s2 = &s[3..];
    // 如果在这里修改会报错，因为存在 s1,s2 不可变引用，违反所有权的原则
    // s.clear();
    println!("s1: {}, s2: {}", s1, s2);
    // 这里 s1,s2 不再使用，可以安全进行可变引用的操作
    s.clear();

    // 数组仍然有切片类型，和字符串切片的原理一样
    let a = [1, 2, 3, 4, 5];
    let slice = &a[1..3];
    let b = [2, 3];
    assert_eq!(slice, &b);

    // 数组的元素是 String 时，想创建所有元素都相同的数组，
    // 由于 String 是在堆上创建存在所有权，由于没有实现 Copy trait，所以无法直接通过声明式创建
    // let str_array: [String; 3] = [String::from("good"); 3];
    // 对于这些堆上的类型可以采用下面填充创建的方法 
    let str_array: [String; 3] = core::array::from_fn(|_| String::from("good"));
    println!("{:?}", str_array);

    // Vec<u8> 的切片类型为 &[u8]
    let vec_a: Vec<u8> = vec![1, 2, 7, 9, 12];
    // 无法直接切片，因为 [u8] 类型的局部变量并没有实现 Sized trait，无法静态确定大小
    // let vec_a_slice = vec_a[..2];
    // 取地址获取的是引用，引用本身局部变量大小是确定的，但是指向的堆内存并不确定
    let vec_a_slice = &vec_a[..3];
    // as_slice 和上面方法等效
    let _vec_a_slice2 = vec_a.as_slice();
    println!("{:?}", vec_a_slice);
    // 通过 .to_vec 或 .to_owned 方法可以转成具备所有权的 Vec 类型
    let vec_b = vec_a_slice.to_vec();
    let vec_c = vec_a_slice.to_owned();
    println!("{:?} {:?}", vec_b, vec_c);

    // &Vec<u8> 可以隐式转换为 &[u8] 这在函数调用时会带来很大的方便
    // 推广来说 &Vec<T> 可以隐式转换为 &[T]
    let _vec_c_slice: &[u8] = &vec_c;
}
