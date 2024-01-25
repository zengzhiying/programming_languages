use std::{thread, time::Duration};

#[derive(Debug, PartialEq, Clone, Copy)]
enum ShirtColor {
    Red,
    Blue,
}

struct Inventory {
    shirts: Vec<ShirtColor>
}

impl Inventory {
    fn giveaway(&self, user_preference: Option<ShirtColor>) -> ShirtColor {
        // 如果用户指定则直接返回，否则则计算当前最多的颜色
        // 这里使用了闭包来获取，并且闭包本身没有传递参数，这种情况下闭包会捕获当前的环境
        // 闭包这里捕获了 Inventory 实例的不可变引用 &self，所以可以直接使用它，但是函数就没有这个功能
        user_preference.unwrap_or_else(|| self.most_stocked())
    }

    /**
     * 计算 shirts 里面哪个颜色的比较多
     */
    fn most_stocked(&self) -> ShirtColor {
        let mut num_red = 0;
        let mut num_blue = 0;
        for color in &self.shirts {
            match color {
                ShirtColor::Blue => num_blue += 1,
                ShirtColor::Red => num_red += 1,
            }
        }

        if num_red > num_blue {
            ShirtColor::Red
        } else {
            ShirtColor::Blue
        }
    }
}


pub fn closures_example() {
    let store = Inventory {
        shirts: vec![ShirtColor::Red, ShirtColor::Blue, ShirtColor::Red]
    };
    let user_pref1 = Some(ShirtColor::Blue);
    let giveaway1 = store.giveaway(user_pref1);
    println!(
        "The user with preference {:?} gets {:?}",
        user_pref1, giveaway1
    );

    let user_pref2 = None;
    let giveaway2 = store.giveaway(user_pref2);
    println!(
        "The user with preference {:?} gets {:?}",
        user_pref2, giveaway2
    );

    // 闭包还有一个特点就是不需要加类型注解，但是函数必须添加类型注解，因为函数要暴露外部使用而闭包不需要
    // 闭包的类型通常由编译器自动推断，因为闭包通常只关联小范围的上下文，所以可以可靠的进行类型推断
    let exp_closures1 = |num| {
        println!("calculating slowly...");
        thread::sleep(Duration::from_secs(1));
        num + 1
    };
    // 调用时编译器会自动推断
    println!("res: {}", exp_closures1(1));
    // 也可以自己注明类型，声明后不使用也不会报错
    let exp_closures2 = |num: i32| -> i32 {
        println!("calculating slowly...");
        thread::sleep(Duration::from_secs(1));
        num * 2
    };
    println!("res: {}", exp_closures2(3));

    // 闭包和函数其实具备演进的这种形式
    // fn  add_one_v1   (x: u32) -> u32 { x + 1 }
    // let add_one_v2 = |x: u32| -> u32 { x + 1 };
    // let add_one_v3 = |x|             { x + 1 };
    // let add_one_v4 = |x|               x + 1  ;

    let exp_closures3 = |x| x;
    // 第一次调用时会自动推断类型
    exp_closures3(String::from("hello"));
    // 之后类型就固定了，调用时不可再变化，否则会报错
    // exp_closures3(3);

    // 上面提到过，默认闭包可以捕获不可变引用
    let ls = vec![1, 2, 3];
    let exp_closures4 = || println!("ls vec: {:?}", ls);
    exp_closures4();
    // 如果要捕获可变引用，则只需定义可变引用
    let mut ls = vec![1,2,3];
    // 闭包也要声明为可变
    let mut exp_closures5 = || ls.push(7);
    // 这里不能加打印，因为可变引用和不可变引用不能同时存在
    // println!("ls vec: {:?}", ls);
    exp_closures5();
    // 执行完之后，可变引用结束，则可以打印
    println!("ls vec: {:?}", ls);

    // 闭包默认不严格需要所有权，如果需要闭包获取所有权，需要使用 move 关键字
    // 这在多线程中经常用，因为多线程存在并发安全的问题，移动所有权可以保证只有线程拥有它
    let ls = vec![1,2,3];
    thread::spawn(move || println!("thread ls vec: {:?}", ls)).join().unwrap();
    // 这里无法再使用，因为所有权已经转移到线程中
    // println!("ls vec: {:?}", ls);

}