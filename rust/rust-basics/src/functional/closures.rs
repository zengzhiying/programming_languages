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

    let exp_closures3 = |x: String| x;
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

    // 闭包函数有 3 种类型，函数或结构体可以通过实现闭包的 trait 来声明接收哪种类型的闭包参数
    // 1.FnOnce 这种适用于只能被调用一次的闭包，所有的闭包都至少实现了这个 trait，因为所有闭包都能被调用。
    // FnOnce 捕获的是不可变引用。
    // 2.FnMut 适用于捕获可变引用的情况，所以它可能会修改被捕获的可变引用，这类闭包可以被调用多次。
    // 3.Fn 也是捕获不可变引用，它可以被调用多次，并且不会改变它们的上下文环境，这在会多次并发调用闭包的场景中十分重要。
    // 这里说法和官网上略有不同，官网上说的意思应该是会捕获所有权并且再返回出去，但是又存在不自洽的地方，所以这里并没有完全按照官网的思路来，
    // 关于这点先保持开放性思考，具体可以参考：
    // https://kaisery.github.io/trpl-zh-cn/ch13-01-closures.html#%E5%B0%86%E8%A2%AB%E6%8D%95%E8%8E%B7%E7%9A%84%E5%80%BC%E7%A7%BB%E5%87%BA%E9%97%AD%E5%8C%85%E5%92%8C-fn-trait

    // 其实 Option<T> unwrap_or_else 方法的声明如下
    // impl<T> Option<T> {
    //     pub fn unwrap_or_else<F>(self, f: F) -> T
    //     where
    //         F: FnOnce() -> T
    //     {
    //         match self {
    //             Some(x) => x,
    //             None => f(),
    //         }
    //     }
    // }
    // 这里 F 要求必须实现 FnOnce，表示最多调用一次，也是使用最宽泛的闭包

    // 在 slice 上定义的方法 sort_by_key 就要求 F 实现 FnMut 
    // sort_by_key 的签名是
    // pub fn sort_by_key<K, F>(&mut self, mut f: F)
    // where
    //     F: FnMut(&T) -> K,
    //     K: Ord,
    // 比如下面的示例代码
    #[derive(Debug)]
    #[allow(dead_code)]
    struct Rectangle {
        width: u32,
        height: u32,
    }

    let mut rects = [
        Rectangle{width: 10, height: 1}, 
        Rectangle{width: 3, height: 5}, 
        Rectangle{width: 7, height: 12},
    ];
    rects.sort_by_key(|r| r.width);
    println!("rects: {:?}", rects);

    // 所以 sort_by_key 传进去的闭包会被多次调用
    // 比如下面的代码就会报错
    let mut opers = vec![];
    // let value = String::from("sort by key");

    // 由于 value 是所有权变量，放到 opers 后所有权就被转移了，所以后面就无法再次调用了
    // 一定要注意：value 的所有权是在 push 时被转移，而并不是在闭包中被转移，这和普通的所有权转移并没有什么不同，
    // 仅仅是因为闭包多次执行引发了这个问题，同时闭包在没有使用 move 时也不会发生所有权转移
    // rects.sort_by_key(|r|{
    //     opers.push(value);
    //     r.width
    // });


    let value = 1;
    let value2 = String::from("sort by key");
    // 这里 value 是值，实现了 Copy trait，所以可以多次放入 opers
    // FnMut 对不可变引用和 Fn 没什么区别，value2 多次使用也不会有什么影响
    // FnMut 只会捕获可变的引用，这里 opers 是可变的具有所有权变量，但是 FnMut 只会捕获其引用
    rects.sort_by_key(|r| {
        opers.push(value);
        println!("value2: {}", value2);
        r.width
    });
    // 执行完成后外部可再次访问
    println!("opers: {:?} value2: {}", opers, value2);

    let mut num = 0;
    // 这里捕获普通的可变引用，也可以多次调用
    rects.sort_by_key(|r| {
        num += 1;
        r.width
    });
    println!("oper num: {}", num);
}