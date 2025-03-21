use std::vec::Vec;
use std::time::{Duration, Instant};

#[derive(Debug)]
enum Direction {
    East,
    West,
    North,
    South,
}

pub fn if_else_control() {
    // if else 多个组合使用，条件中必须使用布尔值
    let n = 6;
    if n % 4 == 0 {
        println!("number {} is divisible by 4", n);
    } else if n % 3 == 0 {
        println!("number {} is divisible by 3", n);
    } else if n % 2 == 0 {
        println!("number {} is divisible by 2", n);
    } else {
        println!("number {} is not divisible by 4, 3, or 2", n);
    }

    let n = 67732;

    // if 语句块同时也可以作为表达式，可以返回值
    let b = if is_even(n) {
        "even"
    } else {
        "odd"
    };

    println!("{} is {}", n , b);
}

pub fn for_control() {
    // 使用 for 迭代集合
    // 用 while 很容易出现越界，因为运行时的越界检查会导致 panic，所以程序会变慢
    // 使用 for 则不会
    let mut vec = Vec::new();
    // 从 1 循环至 10 包括 10
    for i in 1..=10 {
        print!("{} ", i);
    }
    println!();
    // 反转循环
    for i in (1..=10).rev() {
        print!("{} ", i);
    }
    println!();
    // 从 1 循环至 10 不包括 10
    for i in 1..10 {
        vec.push(i);
    }

    // 迭代每个元素的不可变引用
    // 等同于: vec.iter()
    for v in &vec {
        print!("{} ", v);
    }
    println!();

    // 迭代时带上下标
    for (i, v) in vec.iter().enumerate() {
        print!("({}, {}) ", i, v);
    }
    println!();

    println!("{:?}", vec);
    
    // 迭代可变引用, 等同于： vec.iter_mut()
    for v in &mut vec {
        print!("{} ", v);
        // 修改值 使用*解引用 和指针用法类似
        *v += 1;
    }
    println!();

    println!("{:?}", vec);

    // 按照索引迭代
    for i in 0..vec.len() {
        print!("{} ", vec[i]);
    }
    println!();

    // 默认迭代完集合即失效
    // 但是迭代数组没问题, 因为数据同样是在栈上
    for v in vec {
        print!("{} ", v);
    }
    println!();

    // value borrowed here after move
    // println!("{:?}", vec);
}

pub fn while_control() {
    let mut samples = vec![];
    while samples.len() < 10 {
        samples.push(1);
    }

    println!("{:?}", samples);

    // 测试计算机每秒循环次数
    let mut count = 0;
    let time_limit = Duration::new(1, 0);
    // 获取当前时间
    let start = Instant::now();

    // 不过每次获取时间也是存在损耗的
    while (Instant::now() - start) <= time_limit {
        count += 1;
    }

    println!("{}", count);
}

pub fn break_control() {
    // 使用break可以退出循环
    for (x, y) in (0..).zip(0..) {
        if x + y > 100 {
            println!("{},{} break.", x, y);
            break;
        }
    }

    // 对于嵌套循环可以使用标签退出
    'outer: for x in 0.. {
        for y in 0..100 {
            for z in 0..10  {
                if x + y + z > 1000 {
                    println!("{}, {}, {} break outer.", x, y, z);
                    break 'outer;
                }
                
            }
        }
    }
}

pub fn loop_control() {
    // 无限循环 loop 通常会和 break 配合使用
    // loop + break 可以返回值, 因为 loop 本身是表达式
    let mut c = 0;
    let res = loop {
        c += 1;
        if c == 10 {
            break c * 2;
        }
    };
    println!("break res: {}", res);
}

pub fn match_control() {
    let n = 67732;

    // match 本身可以作为表达式返回
    // 语法是 模式 => 执行的代码，多个模式之间用逗号分割
    // 如果每个模式有多行代码需要用大括号 {} 放置执行的代码，后面可以不用再加逗号了
    let b = match is_even(n) {
        true => "even",
        false => "odd",
    };

    println!("{} is {}", n, b);

    // 使用 match 更加安全，编译器会对没有匹配到的情况报错，也就是说匹配是穷尽的
    // match 类似于 switch，但是会更加灵活，支持任意的类型匹配
    let arr = [1, 2, 1, 5, 7, 63, 59, 0];
    for v in arr {
        let r = match v {
            // 单值匹配
            0 => "zero",
            // 多值匹配 或
            63 | 59 => "big",
            // 范围匹配
            7..=10 => "small",
            // 所有其他情况
            _ => "miss",
            // 之后不能再添加任何模式，因为都不会被执行
        };

        println!("{} is {}", v, r);
    }

    // 更强大的枚举值匹配
    let d = Direction::East;
    match_direction(d);
    let d = Direction::North;
    match_direction(d);
    let d = Direction::South;
    match_direction(d);
    
    #[derive(Debug)]
    #[allow(dead_code)]  // 允许无效的变量定义(dead_code)
    enum Action {
        Say(String),
        Move(i32, i32),
        Color(u8, u8, u8)
    }

    let actions = [Action::Say(String::from("Hello")), Action::Move(3, 6), Action::Color(102, 171, 203)];
    for act in actions {
        match act {
            Action::Say(s) => {
                println!("say: {}", s);
            },
            Action::Move(mut x, y) => {
                x += 3;
                println!("x: {}, y: {}", x, y);
            },
            Action::Color(r, g, _) => {
                println!("r: {}, g: {}", r, g);
            }

        }
    }

    // 当只需要匹配一个条件而忽略其他条件时用 if let，其他情况都用 match
    let v = Some(3);
    if let Some(i) = v {
        println!("v is {}.", i);
    }

    let v = Direction::East;
    if let Direction::East = v {
        println!("v is East");
    }

    // while let 循环
    let mut stack: Vec<i32> = Vec::new();
    stack.push(3);
    stack.push(2);
    stack.push(1);

    while let Some(top) = stack.pop() {
        println!("top value: {}", top);
    }

    println!("{:?}", stack);

    // 匹配结构体字段
    #[derive(Debug)]
    struct User { name: String, age: u32, student: bool}
    let u = User { name: String::from("mike"), age: 20, student: false, };
    let User {name, age, student} = u;

    // 匹配后所有权从 u 转移到各个变量，目前会将 name 转移出去，其余字段还可以使用
    println!("user name: {}, age: {}, student: {}", name, age, student);

    // 不转移所有权写法
    let mut u = User { name: String::from("mike"), age: 20, student: false, };
    // 通过 ref 获取引用，mut ref 获取可变引用，前提结构体需要是可变的
    let User {ref mut name, age, student} = u;
    println!("user name: {}, age: {}, student: {}", name, age, student);
    println!("{:?}", u);
    

    // 使用 matches! 进行比较，matches! 返回布尔值
    let v = vec![Direction::West, Direction::South, Direction::East];
    let r = v.iter().filter(|x| matches!(x, Direction::East));
    for fr in r {
        println!("{:?}", fr);
    }

    assert!(matches!('f', 'A'..='Z'|'a'..='z'));
    let v = Some(3);
    assert!(matches!(v, Some(x) if x > 2));
    
}

fn is_even(n: i32) -> bool {
    // 基于表达式返回 只有true和false 不存在其他隐式判断
    n % 2 == 0
}

fn match_direction(d: Direction) {
    match d {
        Direction::East => println!("Direction: East"),
        Direction::West | Direction::North => {
            println!("Direction: West or North.");
        },
        _ => println!("Direction: South"),
    }
}