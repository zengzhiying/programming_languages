use std::{fmt::{Debug, Display}, ops::Add};


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
#[derive(Debug)]
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

// 函数返回 trait 类型
// 但是实际的类型只能返回一种, 无法通过分支控制来返回多个类型, 例如: Weixin 和 Weibo
fn summarizable() -> impl Summary {
    Weibo {
        user: String::from("uid2991"),
        content: String::from("会有突如其来的好运.")
    }
}

// 实现加法 trait
// 最简单的方法可以通过注解实现 Clone+Copy
// #[derive(Debug, Clone, Copy)]
#[derive(Debug)]
struct ThreeDimVector<T: Add<T, Output = T> + Display + Copy> {
    x: T,
    y: T,
    z: T
}


// 实现 Copy
impl<T: Add<T, Output = T> + Display + Copy> Copy for ThreeDimVector<T> {
}

// 实现 Copy 必须实现 Clone
impl<T: Add<T, Output = T> + Display + Copy> Clone for ThreeDimVector<T> {
    fn clone(&self) -> Self {
        *self
    }
}

// 实现 display  所有的实现声明都要和结构体的类型声明一致, 类型约束可以多不可以少
impl<T: Add<T, Output = T> + Display + Copy> Display for ThreeDimVector<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {}, {})", self.x, self.y, self.z)
    }
}

// 实现 add
impl<T: Add<T, Output = T> + Display + Copy> Add for ThreeDimVector<T> {
    type Output = ThreeDimVector<T>;

    fn add(self, v: Self) -> Self::Output {
        ThreeDimVector {
            x: self.x + v.x,
            y: self.y + v.y,
            z: self.z + v.z
        }
    }
}

// =========== 同一个对象不同特征的同名方法 ===========
trait Pilot {
    fn fly(&self);
}
trait Wizard {
    fn fly(&self);
}
struct Human;

impl Pilot for Human {
    fn fly(&self) {
        println!("pilot method.");
    }
}

impl Wizard for Human {
    fn fly(&self) {
        println!("wizard method.");
    }
}

impl Human {
    fn fly(&self) {
        println!("human method.");
    }
}

fn pilot_method(p: &impl Pilot) {
    p.fly();
}
fn wizard_method(p: &impl Wizard) {
    p.fly();
}

// 无 self 参数的同名方法
trait Animal {
    fn baby_name() -> String;
}

struct Dog;
impl Dog {
    fn baby_name() -> String {
        String::from("Spot")
    }
}

impl Animal for Dog {
    fn baby_name() -> String {
        String::from("puppy")
    }
}


// ========== 特征定义时的特征约束 =============
trait OutlinePrint: Display {
    fn print(&self);
}

struct PointPrint {
    x: i32,
    y: i32
}

impl Display for PointPrint {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

// 必须先实现 Display trait
impl OutlinePrint for PointPrint {
    fn print(&self) {
        let output = self.to_string();
        let len = output.len();
        println!("{}", "*".repeat(len + 4));
        println!("*{}*", " ".repeat(len + 2));
        println!("* {} *", output);
        println!("*{}*", " ".repeat(len + 2));
        println!("{}", "*".repeat(len + 4));
    }
}

// 可以使用 Wrapper 元组 newtype 模式绕过孤儿规则, 从而为内部类型实现内部特征
struct Wrapper(Vec<String>);

impl Display for Wrapper {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0.join(", "))
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

    let wb = summarizable();
    println!("return impl: {}", wb.post());

    let v1 = ThreeDimVector{x: 3, y: 6, z: 1};
    let v2 = ThreeDimVector{x: 2, y: 3, z: 0};
    let v3 = v1 + v2;
    println!("v3: {}", v3);
    // 实现 copy 后所有权不会移动
    println!("v1: {}, v2: {}", v1, v2);


    // 同名方法
    let person = Human;
    // 结构体方法
    person.fly();
    // 特征同名方法
    pilot_method(&person);
    wizard_method(&person);
    // 或者不用包装函数
    Pilot::fly(&person);
    Wizard::fly(&person);

    // 无 self 参数的同名方法
    println!("dog: {}", Dog::baby_name());
    // Animal 特征的方法 需要完全限定实例参数
    // <Type as Trait>::function(...)
    println!("animal: {}", <Dog as Animal>::baby_name());

    let pp = PointPrint{x: 9, y: 3};
    pp.print();


    // newtype 模式
    let w = Wrapper(vec![String::from("Hello"), String::from("Rust!")]);
    println!("{}", w);
}
