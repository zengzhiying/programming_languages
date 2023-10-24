
// 枚举类型，非常适合同一类但互斥的事物表示，即一次只能有一种情况
// 枚举类型实例具有所有权，行为都符合所有权规则
#[derive(Debug)]
enum IpAddrKind {
    // 可以在里面包含类型，从而和数据集成在一起
    // 并且每个枚举成员的数据类型都可以不同
    V4(String),
    V6(String)
}

#[derive(Debug)]
enum PokerSuit {
    Clubs, 
    Spades(u8),  // 附带数字
    Diamonds(char), // 附带字符
    Hearts,
    // 可以附带更复杂的结构体
    King{big: bool},
}

pub fn enum_type() {
    // 创建实例，枚举值的类型就是枚举类型，枚举成员的写法和命名空间类似
    // 枚举成员也会自动实现构造方法从而可以传入数据
    let ipv4 = IpAddrKind::V4(String::from("127.0.0.1"));
    let ipv6 = IpAddrKind::V6(String::from("::1"));
    println!("ipv4: {:?}, ipv6: {:?}", ipv4, ipv6);

    let heart = PokerSuit::Hearts;
    let club = PokerSuit::Clubs;

    println!("{:?}, {:?}", heart, club);

    let spade = PokerSuit::Spades(3);
    let diamond = PokerSuit::Diamonds('J');

    println!("{:?}, {:?}", spade, diamond);

    let king = PokerSuit::King { big: true };
    println!("{:?}", king);
    if let PokerSuit::King { big } = king {
        println!("big: {}", big);
    }
}
