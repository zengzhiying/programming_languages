use std::vec::Vec;
use std::time::{Duration, Instant};

pub fn for_control() {
    // 迭代集合
    let mut vec = Vec::new();
    for i in 1..10 {
        vec.push(i);
    }

    // 迭代每个元素的引用
    for v in &vec {
        println!("{}", v);
    }

    println!("{:?}", vec);
    
    for v in &mut vec {
        println!("{}", v);
        // 修改值 使用*解引用 和指针用法类似
        *v += 1;
    }

    println!("{:?}", vec);

    // 按照索引迭代
    for i in 0..vec.len() {
        println!("{}", vec[i]);
    }

    // 默认迭代完集合即失效
    for v in vec {
        println!("{}", v);
    }

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


pub fn if_else_control() {
    let n = 67732;

    let b = if is_even(n) {
        "even"
    } else {
        "odd"
    };

    println!("{} is {}", n , b);
}

pub fn match_control() {
    let n = 67732;

    let b = match is_even(n) {
        true => "even",
        false => "odd",
    };

    println!("{} is {}", n, b);

    // 使用match更加安全 会警告没有考虑到的情况
    // match类似于switch 但是会更加灵活
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
        };

        println!("{} is {}", v, r);
    }
}

fn is_even(n: i32) -> bool {
    // 基于表达式返回 只有true和false 不存在其他隐式判断
    n % 2 == 0
}