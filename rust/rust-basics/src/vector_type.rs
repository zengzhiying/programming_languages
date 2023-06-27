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

    // 元素借用 不能在修改 vector 的同时存在不可变引用
    let mut v = vec![1, 2, 3];
    // let one = &v[0];
    v.push(6);
    // 不打印使用则没问题 rust 会自动释放掉不可变引用
    // println!("{}", one);
    // 只读遍历
    for n in &v {
        println!("{}", n);
    }

    // 遍历过程修改元素值
    for n in &mut v {
        *n += 8;
    }
    println!("{:?}", v);

    // 通过枚举来存储多种类型的元素
    let ips = vec![IpAddr::V4("127.0.0.1".to_string()), IpAddr::V6("::1".to_string())];
    // let ipv4 = &ips[0];
    for ip in ips {
        println!("IP: {:?}", ip);
    }
    // 存在只读引用时也不可直接遍历
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