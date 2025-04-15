use std::{fmt::{Debug, Display}, ops::Add};


// 定义 trait(特征)，trait 的本质是约束，其实常见的类型 u8、f64、String、Vec 或者结构体枚举等具体的类型都是对变量的约束
// 类型系统的约束比较严格，因此就出现了泛型，但是泛型又过于宽泛，所以采用 trait 可以更加灵活的控制类型的范围
// trait 体现了组合优于继承的思想，所有权和 trait 是 Rust 世界中的两大核心部分。
trait Summary {
    // 只定义没有实现的方法，必须要在具体的类型中实现
    // fn post(&self) -> String;
    // 也可以定义默认实现，当类型不实现该方法时将调用默认方法，但是当类型重载方法后将调用类型重载后的方法
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
// 这样使用类型 T 要求 item1 和 item2 的原始类型必须一致, 而且保证实现 Summary trait，但是仅使用 trait 作为参数是无法实现这种约束的
// 因为 T 是声明一种类型，且这种类型实现了 Summary trait，因此在同一个函数同一次调用中必须一致
fn notify2<T: Summary>(item1: &T, item2: &T) {
    println!("notify2: {} {}", item1.post(), item2.post());
}

// 多重约束 使用 + 语法实现
fn notify3(item: &(impl Summary + Debug)) {
    println!("notify3: {}", item.post());
}

// 多重特征约束的泛型实现
fn notify4<T: Summary + Debug>(item1: &T, item2: &T) {
    println!("notify4: {} {}", item1.post(), item2.post());
}

// 通过 where 关键字来约束特征，当类型比较多且每个类型都有不同的 trait bound 时建议使用，这样看起来比较简洁
fn notify5<T, U>(item1: &T, item2: &U) 
    where T: Summary, U: Display
{
    println!("notify5: {} {}", item1.post(), item2);
}

// 函数返回 trait 类型
// 但是实际的类型只能返回一种, 无法通过分支控制来返回多个类型, 例如: Weixin 和 Weibo
fn summarizable() -> impl Summary {
    Weibo {
        user: String::from("uid2991"),
        content: String::from("会有突如其来的好运.")
    }
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
// 注意特征约束不是继承，例如下面的 trait 表示如果类型要实现 OutlinePrint 则必须要实现 Display 才可以
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

// trait 关联类型，trait 中不仅可以定义方法约束还可以定义用于占位的类型
// 这个类型可以用在其中定义的函数参数，也可以用于返回值中
trait Sport {
    // 定义关联类型，在实现时需要指定
    type SoprtType;
    
    // 定义方法约束，关联类型可以写作：Self::SportType 或 <Self as Sport>::SportType
    fn play(&self, st: <Self as Sport>::SoprtType);
    // 返回值使用关联类型
    fn get_sport_type(&self) -> Self::SoprtType;
}

// 定义结构体实现 Sport trait
struct Football;
#[derive(Debug)]
enum SportForm {
    Land,
    Water
}

// 为 Football 实现 Sport trait
impl Sport for Football {
    // 重新设置类型为枚举类型
    type SoprtType = SportForm;

    // 关联类型两种写法：Self::SportType，<Football as Sport>::SportType
    fn play(&self, st: <Football as Sport>::SoprtType) {
        match st {
            SportForm::Land => println!("Land sport"),
            SportForm::Water => println!("Water sport")
        }
    }

    fn get_sport_type(&self) -> Self::SoprtType {
        SportForm::Land
    }
}

// 带有关联类型的 trait，在其他结构体使用其作为约束时可以指定具体的关联类型
// 不指定关联类型直接写即可
/**
 * struct Basketball<T: Sport> {
 *   x: T
 * }
 */
struct Basketball<T: Sport<SoprtType = String>> {
    x: T
}

struct Cba;
impl Sport for Cba {
    type SoprtType = String;
    fn get_sport_type(&self) -> Self::SoprtType {
        "CBA".to_string()
    }
    fn play(&self, st: <Self as Sport>::SoprtType) {
        println!("{}", st);
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

    let wb = summarizable();
    println!("return impl: {}", wb.post());

    let p = Pair::new(3, 8);
    p.cmp_display();
    

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

    // trait 关联类型
    let foot_ball = Football;
    foot_ball.play(SportForm::Land);
    foot_ball.play(SportForm::Water);
    println!("{:?}", foot_ball.get_sport_type());

    // trait 关联类型定义约束
    let basketball = Basketball{
        x: Cba
    };
    basketball.x.play("Basketball Game".to_string());
    println!("{}", basketball.x.get_sport_type());
}
