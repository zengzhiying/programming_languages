#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32
}

impl Rectangle {
    // new 函数不是结构体方法, 而是结构体的关联函数
    // 因为没有 self 实例参数, 调用时按照命名空间方式调用, 即: Rectangle::new
    fn new(width: u32, height: u32) -> Self {
        Rectangle { width, height }
    }

    // 结构体方法
    // 调用时和普通的函数传参一样符合所有权转移
    // 参数可以是 self, &self, & mut self, 调用时不需要严格按照参数定义的实例类型调用, Rust 会自动解构
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn contain(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

// 如果方法太多, 为了美观可以写多个块, 并不影响实现
impl Rectangle {
    fn perimeter(&self) -> u32 {
        2 * (self.width + self.height)
    }
}

#[derive(Debug)]
enum Message {
    Open,
    Write(String),
    Quit(u32)
}

impl Message {
    fn get_message_length(&self) -> usize {
        // if let 对于不可变引用借用, 也只能使用其中变量的引用, 
        // 而不可能转移其内部的所有权使原来对象改变
        if let Message::Write(s) = self {
            return s.len();
        }
        0
    }

    fn get_quit_num(&self) -> u32 {
        if let Message::Quit(n) = self {
            return *n;
        }
        0
    }
}

pub fn struct_method() {
    let rect = Rectangle::new(10, 5);
    // 调用时 rect 会自动结构为结构体方法的参数, 即: &rect
    let area = rect.area();
    println!("rectangle: {:?} area: {} perimeter: {}", rect, area, rect.perimeter());

    let rect1 = Rectangle{width: 15, height: 10};
    let rect2 = Rectangle{width: 8, height: 4};
    println!("rect contain rect1? {}", rect.contain(&rect1));
    println!("rect contain rect2? {}", rect.contain(&rect2));
}

pub fn enum_method() {
    let msg1 = Message::Open;
    let msg2 = Message::Write(String::from("yes"));
    let msg3 = Message::Quit(7);
    println!(
        "{}, {}, {}", 
        msg1.get_message_length(), 
        msg2.get_message_length(), 
        msg3.get_quit_num());
    println!("{:?}, {:?}, {:?}", msg1, msg2, msg3);
}

