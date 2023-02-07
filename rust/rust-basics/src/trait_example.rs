use std::fmt::{Debug, Display};


// 定义 trait(特征)
trait Summary {
    // fn post(&self) -> String;
    // 也可以定义默认实现
    fn post(&self) -> String {
        String::from("Default post.")
    }
}

#[derive(Debug)]
struct Weibo {
    user: String,
    content: String
}

// 实现特征方法
impl Summary for Weibo {
    fn post(&self) -> String {
        format!("微博用户: {}, 内容: {}", self.user, self.content)
    }
}

#[derive(Debug)]
struct Weixin {
    src: String,
    dest: String,
    message: String
}

impl Summary for Weixin {
    fn post(&self) -> String {
        format!("微信用户 {} 发送至 {}, 内容: {}", self.src, self.dest, self.message)
    }
}

#[derive(Debug)]
struct Twitter {}

// 默认实现方式
impl Summary for Twitter {}

// 使用特征作为函数参数 这样只要实现了特征的类型都可以传入 这就实现了所谓的多态
fn notify(item: &impl Summary) {
    println!("notify: {}", item.post());
}

// 特征约束 trait bound
// 这样使用类型 T 要求 item1 和 item2 的原始类型必须一致, 而且保证实现 Summary trait
fn notify2<T: Summary>(item1: &T, item2: &T) {
    println!("notify2: {} {}", item1.post(), item2.post());
}

// 多重约束
fn notify3(item: &(impl Summary + Debug)) {
    println!("notify3: {}", item.post());
}

// 多重约束 + 特征约束
fn notify4<T: Summary + Debug>(item1: &T, item2: &T) {
    println!("notify4: {} {}", item1.post(), item2.post());
}

// 通过 where 关键字来约束特征 特征参数比较多的时候适合使用
fn notify5<T, U>(item1: &T, item2: &U) 
    where T: Summary, U: Display
{
    println!("notify5: {} {}", item1.post(), item2);
}

// 利用特征约束有条件的实现指定的特征
struct Pair<T> {
    x: T,
    y: T
}

impl<T> Pair<T> {
    fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

// 类型 T 必须同时实现 Display 和 PartialOrd trait, 才可以调用其中的方法
impl<T: Display + PartialOrd> Pair<T> {
    fn cmp_display(&self) {
        if self.x >= self.y {
            println!("maxinum is: {}", self.x);
        } else {
            println!("maxinum is: {}", self.y);
        }
    }
}

pub fn example() {
    let weibo = Weibo{user: String::from("u1800"), content: String::from("昨天烟花不错.")};
    let weixin = Weixin{src: String::from("wxid01"), dest: String::from("wxid02"), message: String::from("我是马化腾")};
    let twitter = Twitter{};
    println!("{:?}, {}", weibo, weibo.post());
    println!("{:?}, {}", weixin, weixin.post());
    println!("{:?}, {}", twitter, twitter.post());

    notify(&weibo);
    notify(&weixin);
    notify(&twitter);

    let weibo2 = Weibo{user: String::from("u1902"), content: String::from("今天天气不错.")};
    notify2(&weibo, &weibo2);

    notify3(&weixin);

    notify4(&weibo, &weibo2);

    notify5(&twitter, &10);

    let p = Pair::new(3, 8);
    p.cmp_display();
}
