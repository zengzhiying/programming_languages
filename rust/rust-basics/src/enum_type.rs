
// 枚举类型，非常适合同一类但互斥的事物表示，即一次只能有一种情况
// 枚举类型实例具有所有权，行为都符合所有权规则
#[derive(Debug)]
#[allow(dead_code)]
enum IpAddrKind {
    // 可以在里面包含类型，从而和数据集成在一起
    // 并且每个枚举成员的数据类型都可以不同
    V4(String),
    V6(String)
}

#[derive(Debug)]
#[allow(dead_code)]
struct Queen {
    name: String,
    age: u8
}

#[derive(Debug)]
#[allow(dead_code)]
enum PokerSuit {
    Clubs, 
    Spades(u8),  // 附带数字
    Diamonds(char), // 附带字符
    Hearts,
    // 可以附带更复杂的结构体
    King{big: bool},
    Queen(Queen)
}

// 类 C 或 Go 语言风格的枚举
#[allow(dead_code)]
enum Number {
    // 设置初始值，后续自增
    Zero = 0,
    One,
    Two,
}
// 每个枚举值可以单独赋值
#[allow(dead_code)]
enum Color {
    Red = 0xff0000,
    Green = 0x00ff00,
    Blue = 0x0000ff,
}

// 空枚举，无法被实例化
#[allow(dead_code)]
enum Empty {}

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

    let qe = PokerSuit::Queen(Queen { name: "Eli".to_string(), age: 73 });
    if let PokerSuit::Queen(q) = qe {
        // 所有权被转移到 q，原有的枚举将无法使用
        println!("queen: {:?}", q);
    }

    println!("Number zero is {}, Two is {}", Number::Zero as i32, Number::Two as i32);
    println!("Color blue is #{:06x}", Color::Blue as i32);

    // 枚举通用匹配，match 可以直接返回值
    let ip_addr = match ipv4 {
        IpAddrKind::V4(ip_addr) => {
            println!("ip v4 addr: {}", ip_addr);
            ip_addr
        }
        IpAddrKind::V6(ip_addr) => ip_addr
    };
    println!("ip addr: {}", ip_addr);

    // 可以使用 _ 忽略分支
    let num = Number::Two;
    let num_value = match num {
        Number::Two => {
            2
        }
        _ => 0
    };
    println!("num value: {}", num_value);
}
