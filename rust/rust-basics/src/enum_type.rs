
// 枚举类型
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
    // 枚举值的类型就是枚举类型
    let heart = PokerSuit::Hearts;
    let club = PokerSuit::Clubs;

    println!("{:?}, {:?}", heart, club);

    let spade = PokerSuit::Spades(3);
    let diamond = PokerSuit::Diamonds('J');

    println!("{:?}, {:?}", spade, diamond);

    let king = PokerSuit::King { big: true };
    println!("{:?}", king);
}
