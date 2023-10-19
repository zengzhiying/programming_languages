
// 通常结构体的字段不能包含引用, 每个结构体都有其完整的数据
// 如果包含引用则需要用到生命周期定义, 因为要确保结构体的作用范围小于其中属性的作用范围, 
// 否则在使用结构体的时候会出现属性无效的混乱情况, 因此如果不定义生命周期（lifetimes）是无法在结构体中使用引用的
// 另外结构体的实例本身是符合所有权规则的，除非实现了 Copy trait
// 结构体默认没有实现 Display trait，可以添加注解来实现打印，占位符可以使用：{:?} 以及 {:#?}，其中 {:#?} 做了换行的格式化
#[derive(Debug)]
struct Buffer {
    name: String,
    data: Vec<u8>
}

// 元组结构体，只关心结构体的名字，字段比较简单，
// 元组结构体的实例同样符合所有权规则，而不关心其中的类型是否实现 Copy trait，这点和元组类型不同
// 元组结构体同样没有实现 Display trait，可以加注解实现打印
#[derive(Debug)]
struct Point(i32, i32, i32);

// 类单元结构体（unit-like structs）
// 没有字段, 类似于空的接口, 主要用来绑定方法或实现 trait 使用
#[derive(Debug)]
struct Equal;

fn new_buffer(name: String, mut data: Vec<u8>) -> Buffer {
    data.push(8);
    data.push(9);
    data.push(10);
    // Buffer { name: name, data: data }
    // 如果字段名称和变量名称一致则也可以忽略 不一致的再单独写
    Buffer { name, data }
}

fn print_buffer(buf: &mut Buffer) {
    let j = &mut buf.data;

    j.push(3);

    println!("buffer: {:?} name: {} data: {:?}", buf, buf.name, buf.data);
}

pub fn structure_type() {
    // 如果后续要修改结构体的属性值，则必须对实例使用 mut 声明
    let mut buf = new_buffer(String::from("hello"), Vec::new());
    print_buffer(&mut buf);
    println!("buf: {:?}", buf);
    buf.name = String::from("struct");
    println!("buf: {:?}", buf);
    // 结构体字段较多时可以换行输出 更加美观一些
    println!("buf: {:#?}", buf);
    // 还可以使用 dbg! 不过会转移所有权并且返回所有权, 可以使用引用或者接收所有权
    // buf = dbg!(buf);
    // dbg! 打印的是标准错误流（stderr），而不是标准输出
    dbg!(&buf);
    // dbg! 还可以在设置值的时候打印更详细的过程信息，例如：
    let _a = dbg!(30 * 20);

    // 结构体更新语法，有变化的手动列出其余的从原来的结构体获取，避免去单独设置每个值
    let buf2 = Buffer{name: String::from("update"), ..buf};
    println!("buf2: {:?}", buf2);
    // 此时 buf 中的 data 所有权已经发生变化无法被访问, 但是可以单独访问 buf.name
    // 对于原来的结构体属性如果是实现了 Copy trait，则不影响访问，这完全符合所有权的原理
    println!("buf.name: {:?}", buf.name);

    let p = Point(2, 6, 0);
    println!("point: {:?}, point[1]: {}", p, p.1);

    let e = Equal;
    println!("equal: {:?}", e);
}


// 结构体方法示例
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32
}

impl Rectangle {
    // new 函数不是结构体方法, 而是结构体的关联函数（associated functions），new 也不是关键字，所以可以放心使用
    // 因为没有 self 实例参数, 调用时按照命名空间方式调用, 即: Rectangle::new，这就和 String::from 一样
    // 在 impl 块内 Self 类型是 impl 指定类型的别名
    fn new(width: u32, height: u32) -> Self {
        Rectangle { width, height }
    }

    // 定义结构体方法
    // 调用时和普通的函数传参一样符合所有权转移
    // 参数可以是 self, &self, & mut self, 调用时不需要严格按照参数定义的实例类型调用, Rust 会自动解构
    // &self 是 self: &Self 的简写
    // 这里只需要获取 Self 的属性，所以使用只读引用，如果需要修改 Self 中的属性则可以使用可变引用
    // 直接使用 Self 来转移所有权的方式很少见，这样在调用完方法之后实例就释放了
    fn area(&self) -> u32 {
        self.width * self.height
    }
}

// 如果方法太多, 为了美观可以写多个块, 并不影响实现
impl Rectangle {
    fn contain(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }

    fn perimeter(&self) -> u32 {
        2 * (self.width + self.height)
    }
}

pub fn structure_method() {
    let rect = Rectangle::new(10, 5);
    // 调用时 rect 会自动解构为结构体方法的参数, 即: &rect
    // Rust 并不像 C/C++ 中一样通过 . 和 -> 区分对象上调用方法还是对象指针上调用方法
    // Rust 编译器自动帮我们做了这些尝试，当调用方法时 Rust 会自动为对象添加 &, &mut 或 * 以便为对象定义的方法签名匹配
    // 所以这种隐式操作简化了代码的编写，在使用时更加友好
    // let area = (&rect).area();
    // 直接写和上面调用方式相同
    let area = rect.area();
    println!("rectangle: {:?} area: {} perimeter: {}", rect, area, rect.perimeter());

    let rect1 = Rectangle{width: 15, height: 10};
    let rect2 = Rectangle{width: 8, height: 4};
    println!("rect contain rect1? {}", rect.contain(&rect1));
    println!("rect contain rect2? {}", rect.contain(&rect2));
}


