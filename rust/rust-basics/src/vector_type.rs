#[derive(Debug)]
enum IpAddr {
    V4(String),
    V6(String)
}

trait IpAddress {
    fn display(&self);
}

struct IpV4(String);
impl IpAddress for IpV4 {
    fn display(&self) {
        println!("ipv4: {}", self.0);
    }
}

struct IpV6(String);
impl IpAddress for IpV6 {
    fn display(&self) {
        println!("ipv6: {}", self.0);
    }
}

// Vector 类型是动态类型，数据存储在堆上，符合所有权规则
// 和其他对象的行为一样，比如超出作用域会自动删除变量以及所有的元素
// 借用检查器会保证任何 Vector 的引用只有当 Vector 本身有效时才可用
pub fn vector_type_example() {
    // 创建动态数组 必须指定类型
    let _v1: Vec<i32> = Vec::new();
    // 可变容器如果放入元素, 则无需指定类型, 会自动推断
    let mut v1 = Vec::new();
    // 尾部插入元素，只有可变类型的 Vector 才可以插入元素
    v1.push(3);
    v1.push(4);
    v1.push(5);
    // pop 方法会移除并返回最后一个元素
    assert_eq!(v1.pop().unwrap(), 5);

    // 创建时指定大小, 提升性能
    let mut v1 = Vec::with_capacity(10);
    v1.push(3);

    println!("{:?}", v1);

    // 使用宏 vec! 直接创建 vector, 会自动推断类型
    let v2 = vec![1, 2, 3];
    println!("{:?}", v2);

    // 下标索引访问元素, 注意不要越界，否则会直接 panic
    let n = &v2[2];
    println!("v2[2]: {}", n);
    // get 方式访问, 返回 Option, 越界会返回 None
    match v2.get(2) {
        Some(n) => println!("v2[2]: {}", n),
        None => println!("v2[2] not exists.")
    }

    // 元素借用 不能在修改 vector 的同时存在任何元素的不可变引用，严格符合所有权的规则
    let mut v = vec![1, 2, 3];
    // let one = &v[0];
    v.push(6);
    // 不打印使用则没问题，Rust 会自动释放掉不可变引用
    // 之所以这么设计是为了避免 Vector 扩容时可能会开辟新的空间，原来引用位置的空间将被释放，
    // 这样引用就会变为悬空指针，所以不允许这种情况出现
    // println!("{}", one);

    // 只读遍历，获取的都是不可变引用
    for n in &v {
        println!("{}", n);
    }

    // 遍历过程修改元素值，使用可变引用遍历
    // 无论如何不可变还是可变引用方式遍历，在循环内部都不允许对 Vector 本身进行修改，
    // 这同样违反 Rust 借用的规则即不能同时存在可变与不可变引用以及可变引用同时有且只能有一个
    for n in &mut v {
        *n += 8;
    }
    println!("{:?}", v);

    // 通过枚举类型来间接实现存储多种类型的元素
    let ips = vec![IpAddr::V4("127.0.0.1".to_string()), IpAddr::V6("::1".to_string())];
    // let ipv4 = &ips[0];
    for ip in ips {
        println!("IP: {:?}", ip);
    }
    // 存在上面的只读引用时也不可进行原始的遍历，会涉及到所有权转移，但是可以进行只读遍历
    // println!("{:?}", ipv4);

    // 通过特征以及 Box 动态分发来实现多种结构存储
    let ips: Vec<Box<dyn IpAddress>> = vec![
        Box::new(IpV4("127.0.0.1".to_string())), 
        Box::new(IpV6("::1".to_string()))
    ];

    for ip in ips {
        ip.display();
    }

}